//! Metadata tracking types for handling unknown fields.

/// A type for encoding, paradoxically, which fields are *known*, so that we know which ones to
/// avoid when finding *unknown* fields.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UnknownFields<'a> {
    known_tags: &'static [u32],
    msg_buf: &'a [u8],
    found_unknown_tag: bool,
}

impl<'a> UnknownFields<'a> {
    /// Constructs a new unknown fields struct with the assumptions that all the fields in the
    /// surrounding message are known.
    pub const fn empty() -> Self {
        UnknownFields {
            known_tags: &[],
            msg_buf: &[],
            found_unknown_tag: false,
        }
    }

    /// Constructs a new `UnknownFields` from the specified message buffer.  This constructor is
    /// only used from runtime code.
    ///
    /// The `known_tags` slice is used to determine which tags are known.  The `found_unknown_tag`
    /// is exposed for completeness, but should realistically always be set to `true`, since it only
    /// makes sense to use this constructor if an unknown tag has been encountered.  Otherwise,
    /// it is better to use the empty constructor, `UnknownFields::empty()`.
    pub(crate) fn from_msg_buf(
        known_tags: &'static [u32],
        msg_buf: &'a [u8],
        found_unknown_tag: bool,
    ) -> Self {
        Self {
            known_tags,
            msg_buf,
            found_unknown_tag,
        }
    }

    /// Whether the field has been populated from either deserialization or by the user.  This
    /// method is only used from runtime code.
    ///
    /// Used by the decoding runtime logic for avoiding populating the same field twice for multiple
    /// occurrences of the same field; since `from_msg_buf` takes the entire message buffer as an
    /// argument anyway, there's no sense in populating the field multiple times.
    pub(crate) fn is_unpopulated(&self) -> bool {
        !self.found_unknown_tag
    }
}
