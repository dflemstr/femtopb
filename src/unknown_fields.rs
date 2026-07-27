//! Metadata tracking types for handling unknown fields.

use crate::window;

/// A type for encoding, paradoxically, which fields are *known*, so that we know which ones to
/// avoid when finding *unknown* fields.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UnknownFields<'a> {
    known_tags: &'static [u32],
    /// The span of the surrounding message buffer holding the unknown fields: from the first
    /// unknown field to the end of the last one. Narrowing to this window means re-encoding does not
    /// have to re-scan the (typically large) run of known fields before and after the unknown ones.
    window: window::Window<'a>,
    found_unknown_tag: bool,
}

impl<'a> UnknownFields<'a> {
    /// Constructs a new unknown fields struct with the assumptions that all the fields in the
    /// surrounding message are known.
    pub const fn empty() -> Self {
        UnknownFields {
            known_tags: &[],
            window: window::Window::empty(),
            found_unknown_tag: false,
        }
    }

    /// Anchors the retained region at the first unknown field encountered while decoding. `msg_buf`
    /// is the whole surrounding message buffer and `field_start` (a suffix of it) is where that
    /// field's bytes begin. The span is subsequently grown with [`extend_to`](Self::extend_to) as
    /// more unknown fields are seen.
    ///
    /// Only used from runtime code.
    pub(crate) fn anchored(
        known_tags: &'static [u32],
        msg_buf: &'a [u8],
        field_start: &[u8],
    ) -> Self {
        Self {
            known_tags,
            window: window::Window::anchored(msg_buf, field_start),
            found_unknown_tag: true,
        }
    }

    /// Grows the retained region so that it ends where `remaining` begins — i.e. at the end of the
    /// unknown field that was just skipped. `remaining` must be a suffix of the same message buffer
    /// as the anchored region, so this is only called from runtime code, once per unknown field.
    pub(crate) fn extend_to(&mut self, remaining: &[u8]) {
        self.window.extend_to(remaining);
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
        self.window.region()
    }
}
