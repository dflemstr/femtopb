//! Metadata tracking types for handling unknown fields.

/// A type for encoding, paradoxically, which fields are *known*, so that we know which ones to
/// avoid when finding *unknown* fields.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UnknownFields<'a> {
    known_tags: &'static [u32],
    /// A slice of the surrounding message buffer that begins at the first unknown field. Only its
    /// first `region_len` bytes — the span up to the end of the *last* unknown field — hold
    /// unknown-field data; anything past that is known fields that follow and is ignored. Narrowing
    /// to this span means re-encoding does not have to re-scan the (typically large) run of known
    /// fields before and after the unknown ones.
    region: &'a [u8],
    region_len: usize,
    found_unknown_tag: bool,
}

impl<'a> UnknownFields<'a> {
    /// Constructs a new unknown fields struct with the assumptions that all the fields in the
    /// surrounding message are known.
    pub const fn empty() -> Self {
        UnknownFields {
            known_tags: &[],
            region: &[],
            region_len: 0,
            found_unknown_tag: false,
        }
    }

    /// Anchors the retained region at the first unknown field encountered while decoding, whose
    /// bytes begin at `field_start` (a suffix of the surrounding message buffer). The span is
    /// subsequently grown with [`extend_to`](Self::extend_to) as more unknown fields are seen.
    ///
    /// Only used from runtime code.
    pub(crate) fn anchored(known_tags: &'static [u32], field_start: &'a [u8]) -> Self {
        Self {
            known_tags,
            region: field_start,
            region_len: 0,
            found_unknown_tag: true,
        }
    }

    /// Grows the retained region so that it ends where `remaining` begins — i.e. at the end of the
    /// unknown field that was just skipped. `remaining` must be a suffix of the same message buffer
    /// as the anchored `region`, so this is only called from runtime code, once per unknown field.
    pub(crate) fn extend_to(&mut self, remaining: &[u8]) {
        // `region` and `remaining` are both suffixes of the message buffer; since decoding advances
        // through it in order, `remaining` is never longer than `region`, and the difference is the
        // number of bytes from the first unknown field up to the end of this one.
        self.region_len = self.region.len().saturating_sub(remaining.len());
    }

    /// Whether the field has been populated from either deserialization or by the user.  This
    /// method is only used from runtime code.
    ///
    /// Used by the decoding runtime logic for avoiding re-anchoring the region for every unknown
    /// field; the region is anchored on the first unknown field and only extended thereafter.
    pub(crate) fn is_unpopulated(&self) -> bool {
        !self.found_unknown_tag
    }

    /// The tags that belong to *known* fields of the surrounding message, used from runtime code to
    /// tell known fields apart from the unknown ones when re-encoding.
    pub(crate) fn known_tags(&self) -> &'static [u32] {
        self.known_tags
    }

    /// The span of the surrounding message buffer containing the unknown fields (and possibly some
    /// interleaved known fields, which are filtered out by tag when re-encoding).
    pub(crate) fn region(&self) -> &'a [u8] {
        self.region.get(..self.region_len).unwrap_or(self.region)
    }
}
