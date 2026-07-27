//! A sub-window into a borrowed message buffer.

/// A sub-window of a message buffer: the whole buffer `data`, plus the `start..end` byte offsets of
/// the span of interest within it (e.g. a repeated field's occurrences, or the unknown fields). The
/// bytes outside `start..end` belong to other fields and are ignored (see [`Self::region`]).
///
/// The offsets are kept as two `usize` fields rather than a `Range<usize>` because `Range` is not
/// `Copy`, and this type is embedded in the `Copy` field wrappers.
///
/// Growing the window (see [`Self::extend_to`]) is a flat, unconditional store of `end`, which keeps
/// the decode hot path — where it happens once per occurrence — provably panic-free. Writing through
/// an `enum` variant, or re-deriving a sub-slice there, perturbs the optimizer enough to defeat the
/// crate's no-panic guarantee.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct Window<'a> {
    data: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> Window<'a> {
    /// An empty window that covers nothing.
    pub(crate) const fn empty() -> Self {
        Self {
            data: &[],
            start: 0,
            end: 0,
        }
    }

    /// A window into `data` starting where `from` begins (a suffix of `data`) and covering *nothing*
    /// initially; grow it with [`Self::extend_to`] as decoding walks the bytes of interest. Use this
    /// when the safe fallback for an un-extended window is "empty" (e.g. unknown fields, where an
    /// un-narrowed window would wrongly re-encode trailing known fields).
    pub(crate) fn anchored(data: &'a [u8], from: &[u8]) -> Self {
        let start = data.len().saturating_sub(from.len());
        Self {
            data,
            start,
            end: start,
        }
    }

    /// A window into `data` starting where `from` begins (a suffix of `data`) and covering all the
    /// way to the end of `data`. [`Self::extend_to`] then narrows the tail to end at the last
    /// occurrence. Use this when the safe fallback for an un-extended window is "the whole rest of
    /// the buffer" (e.g. a lazily-parsed repeated/packed field, which simply re-scans to the end —
    /// correct, just not narrowed — if `extend_to` never runs).
    pub(crate) fn covering(data: &'a [u8], from: &[u8]) -> Self {
        let start = data.len().saturating_sub(from.len());
        Self {
            data,
            start,
            end: data.len(),
        }
    }

    /// Sets the window to end where `remaining` begins — i.e. at the end of the occurrence that was
    /// just skipped. `remaining` must be a suffix of `data`; since decoding advances through the
    /// buffer in order, `data.len() - remaining.len()` is the absolute offset of that point. Only
    /// ever called from the decoding runtime, once per occurrence.
    #[inline]
    pub(crate) fn extend_to(&mut self, remaining: &[u8]) {
        self.end = self.data.len().saturating_sub(remaining.len());
    }

    /// The bytes the window covers: `data[start..end]`.
    #[inline]
    #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
    pub(crate) fn region(&self) -> &'a [u8] {
        self.data.get(self.start..self.end).unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_covers_nothing() {
        let w = Window::empty();
        assert_eq!((w.start, w.end), (0, 0));
        assert_eq!(w.region(), b"");
    }

    #[test]
    fn anchored_starts_at_the_field_and_covers_nothing_until_extended() {
        let data = b"abcdef";
        let w = Window::anchored(data, &data[2..]); // `from` begins at offset 2
        assert_eq!((w.start, w.end), (2, 2));
        assert_eq!(w.region(), b"");
    }

    #[test]
    fn covering_spans_from_the_field_to_the_end() {
        let data = b"abcdef";
        let w = Window::covering(data, &data[2..]);
        assert_eq!((w.start, w.end), (2, 6));
        assert_eq!(w.region(), b"cdef");
    }

    #[test]
    fn extend_to_narrows_the_tail() {
        let data = b"abcdef";
        let mut w = Window::covering(data, &data[2..]); // start 2, end 6
        w.extend_to(&data[4..]); // `remaining` begins at offset 4
        assert_eq!((w.start, w.end), (2, 4));
        assert_eq!(w.region(), b"cd");
    }

    #[test]
    fn extend_to_can_be_called_repeatedly() {
        let data = b"abcdef";
        let mut w = Window::covering(data, &data[1..]); // start 1
        w.extend_to(&data[3..]); // end -> 3
        assert_eq!(w.region(), b"bc");
        w.extend_to(&data[5..]); // end -> 5 (grows)
        assert_eq!(w.region(), b"bcde");
    }

    #[test]
    fn anchored_and_covering_saturate_when_from_is_longer_than_data() {
        // A degenerate/hostile `from` longer than `data` must not underflow `start`; it clamps to 0.
        let data = b"ab";
        let longer = b"xxxxx";
        let anchored = Window::anchored(data, longer);
        assert_eq!((anchored.start, anchored.end), (0, 0));
        assert_eq!(anchored.region(), b"");
        let covering = Window::covering(data, longer);
        assert_eq!((covering.start, covering.end), (0, 2));
        assert_eq!(covering.region(), b"ab");
    }

    #[test]
    fn extend_to_saturates_when_remaining_is_longer_than_data() {
        // `remaining` longer than `data` clamps `end` to 0, leaving `start > end`; `region` must
        // return empty via its `get(..).unwrap_or(&[])` guard rather than panicking.
        let data = b"ab";
        let mut w = Window::covering(data, data); // start 0, end 2
        w.extend_to(b"xxxxx");
        assert_eq!(w.end, 0);
        assert_eq!(w.region(), b"");
    }

    #[test]
    fn region_with_start_greater_than_end_is_empty_not_a_panic() {
        let w = Window {
            data: b"abcdef",
            start: 4,
            end: 2,
        };
        assert_eq!(w.region(), b"");
    }

    proptest::proptest! {
        /// `region()` is on the panic-free decode path; it must never panic for *any* combination
        /// of `data`, `start`, and `end` (including out-of-range and inverted offsets), always
        /// returning a sub-slice of `data` or empty.
        #[test]
        fn region_never_panics_for_arbitrary_offsets(
            data: Vec<u8>,
            start in 0usize..64,
            end in 0usize..64,
        ) {
            let w = Window { data: &data, start, end };
            let region = w.region();
            proptest::prop_assert!(region.len() <= data.len());
        }
    }
}
