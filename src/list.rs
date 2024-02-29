#[derive(Default)]
pub enum List<'a, A> {
    #[default]
    Empty,
    MessageBuffer(MessageBuffer<'a>),
    Slice(&'a [A]),
}

#[derive(Clone, Copy, Debug)]
pub struct MessageBuffer<'a> {
    /// The field tag to look for inside `data`.
    ///
    /// The field might occur several times, with other fields before, after, or in between.
    pub tag: u32,
    /// The data of the entire message buffer.
    pub data: &'a [u8],
}

impl<'a, A> List<'a, A> {
    pub const fn empty() -> Self {
        Self::Empty
    }

    pub const fn from_slice(slice: &'a [A]) -> Self {
        List::Slice(slice)
    }

    pub const fn from_msg_buf(tag: u32, data: &'a [u8]) -> Self {
        List::MessageBuffer(MessageBuffer { tag, data })
    }
}

// Implemented manually since `List<'a, A>: Clone` does not require `A: Clone`
impl<'a, A> Clone for List<'a, A> {
    fn clone(&self) -> Self {
        match *self {
            List::Empty => List::Empty,
            List::MessageBuffer(ref b) => List::MessageBuffer(b.clone()),
            List::Slice(s) => List::Slice(s),
        }
    }
}

impl<'a, A> Copy for List<'a, A> {}
