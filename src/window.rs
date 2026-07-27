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
