/// The backing storage of a lazily-parsed `Repeated`/`Packed` field.
///
/// The message-buffer window itself lives *alongside* this enum on the wrapper (as a
/// [`window::Window`](crate::window::Window)), not inside the `MessageBuffer` variant. Keeping the
/// per-occurrence region update a flat, unconditional write on the wrapper — rather than a write
/// through this enum's variant — is what keeps the decode hot path provably panic-free.
#[derive(Default)]
pub enum List<'a, A> {
    #[default]
    Empty,
    /// A field decoded lazily from a message buffer; holds the field's tag. The buffer window it
    /// scans is stored separately on the wrapping `Repeated`/`Packed`.
    MessageBuffer(u32),
    Slice(&'a [A]),
}

/// The cursor an iterator walks: a field tag plus the remaining bytes of the field's window.
#[derive(Clone, Copy, Debug)]
pub struct MessageBuffer<'a> {
    /// The field tag to look for inside `data`.
    ///
    /// The field might occur several times, with other fields before, after, or in between.
    pub tag: u32,
    /// The remaining bytes of the field's window, consumed as the iterator advances.
    pub data: &'a [u8],
}

impl<'a, A> List<'a, A> {
    pub const fn empty() -> Self {
        Self::Empty
    }

    pub const fn from_slice(slice: &'a [A]) -> Self {
        List::Slice(slice)
    }

    pub const fn from_msg_buf(tag: u32) -> Self {
        List::MessageBuffer(tag)
    }
}

// Implemented manually since deriving `Clone` would wrongly require `A: Clone`; `List` is `Copy`
// regardless of `A`, so a plain copy suffices.
impl<'a, A> Clone for List<'a, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, A> Copy for List<'a, A> {}
