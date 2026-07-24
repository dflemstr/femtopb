//! `Repeated` scalar or composite values.
use crate::{encoding, error, item_encoding, list};
use core::marker;
use core::{fmt, slice};

/// A sparse encoding of a sequence of scalar values.
///
/// Use `.iter()` to iterate through the elements of this `Repeated`.
#[repr(transparent)]
pub struct Repeated<'a, A, E>(list::List<'a, A>, marker::PhantomData<E>)
where
    E: item_encoding::ItemEncoding<'a, A>;

/// An iterator for a `Repeated`.
#[derive(Clone, Debug, Default)]
pub struct Iter<'a, A, E>(IterRepr<'a, A, E>)
where
    E: item_encoding::ItemEncoding<'a, A>;

#[derive(Clone, Debug, Default)]
enum IterRepr<'a, A, E>
where
    E: item_encoding::ItemEncoding<'a, A>,
{
    #[default]
    Empty,
    MessageBuffer {
        msg_buf: list::MessageBuffer<'a>,
        /// Remaining bytes of a packed occurrence currently being iterated. A repeated field may
        /// legitimately be encoded using the packed wire format (this is the proto3 default), so we
        /// accept both; this holds the leftover of such a chunk between calls.
        packed_chunk: &'a [u8],
        phantom: marker::PhantomData<E>,
    },
    Slice(slice::Iter<'a, A>),
}

impl<'a, A, E> Repeated<'a, A, E>
where
    E: item_encoding::ItemEncoding<'a, A>,
{
    /// Creates a new, empty `Repeated` with minimal memory footprint.
    #[must_use]
    pub const fn empty() -> Self {
        Self(list::List::empty(), marker::PhantomData)
    }

    /// Creates a `Repeated` that uses the specified slice as its storage.
    ///
    /// The slice must live as long as this `Repeated` does.
    #[must_use]
    pub const fn from_slice(slice: &'a [A]) -> Self {
        Self(list::List::from_slice(slice), marker::PhantomData)
    }

    // Used internally by the runtime during decoding
    #[must_use]
    pub const fn from_msg_buf(tag: u32, data: &'a [u8]) -> Self {
        Self(list::List::from_msg_buf(tag, data), marker::PhantomData)
    }

    /// Whether the field has been populated from either deserialization or by the user.
    ///
    /// Used by the decoding runtime logic for avoiding populating the same field twice for multiple
    /// occurrences of the same field; since `from_msg_buf` takes the entire message buffer as an
    /// argument anyway, there's no sense in populating the field multiple times.
    pub(crate) fn is_unpopulated(&self) -> bool {
        matches!(self.0, list::List::Empty)
    }
}

impl<'a, A, E> Repeated<'a, A, E>
where
    A: Clone,
    E: item_encoding::ItemEncoding<'a, A>,
{
    #[must_use]
    pub fn iter(&self) -> Iter<'a, A, E> {
        self.into_iter()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        // This is different from `self.is_unpopulated()`, because the other reprs
        // (e.g. empty slice, or message buffer without an occurrence of the right tag) might also
        // be empty.
        self.iter().next().is_none() // TODO: optimization potential?
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.iter().count() // TODO: optimization potential?
    }
}

impl<'a, A, E> Iter<'a, A, E>
where
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn from_list(lst: list::List<'a, A>) -> Self {
        let repr = match lst {
            list::List::Empty => IterRepr::Empty,
            list::List::MessageBuffer(msg_buf) => IterRepr::MessageBuffer {
                msg_buf,
                packed_chunk: &[],
                phantom: marker::PhantomData,
            },
            list::List::Slice(slice) => IterRepr::Slice(slice.iter()),
        };
        Self(repr)
    }
}

impl<'a, A, E> PartialEq for Repeated<'a, A, E>
where
    A: Clone + PartialEq,
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

// Implemented manually since deriving `Clone` would wrongly require `A: Clone`; `Repeated` is
// `Copy` regardless of `A`, so a plain copy suffices.
impl<'a, A, E> Clone for Repeated<'a, A, E>
where
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, A, E> Copy for Repeated<'a, A, E> where E: item_encoding::ItemEncoding<'a, A> {}

impl<'a, A, E> Default for Repeated<'a, A, E>
where
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, A, E> fmt::Debug for Repeated<'a, A, E>
where
    A: Clone + fmt::Debug,
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list_fmt = f.debug_list();
        for ref item in self.iter() {
            list_fmt.entry(item);
        }
        list_fmt.finish()
    }
}

#[cfg(feature = "defmt")]
impl<'a, A, E> defmt::Format for Repeated<'a, A, E>
where
    A: Clone + defmt::Format,
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "[");
        for ref item in self.iter() {
            match item {
                Ok(item) => {
                    defmt::write!(fmt, "{:?}", item);
                }
                Err(e) => {
                    defmt::write!(fmt, "...error: {:?}", e);
                    break;
                }
            }
        }
        defmt::write!(fmt, "]");
    }
}

impl<'a, A, E> IntoIterator for Repeated<'a, A, E>
where
    A: Clone,
    E: item_encoding::ItemEncoding<'a, A>,
{
    type Item = Result<A, error::DecodeError>;
    type IntoIter = Iter<'a, A, E>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::from_list(self.0)
    }
}

impl<'a, A, E> IntoIterator for &Repeated<'a, A, E>
where
    A: Clone,
    E: item_encoding::ItemEncoding<'a, A>,
{
    type Item = Result<A, error::DecodeError>;
    type IntoIter = Iter<'a, A, E>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::from_list(self.0)
    }
}

impl<'a, A, E> Iterator for Iter<'a, A, E>
where
    A: Clone,
    E: item_encoding::ItemEncoding<'a, A>,
{
    type Item = Result<A, error::DecodeError>;

    #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            IterRepr::Empty => None,
            IterRepr::MessageBuffer {
                ref mut msg_buf,
                ref mut packed_chunk,
                phantom: _,
            } => {
                let result = next_item::<A, E>(msg_buf, packed_chunk);
                if result.is_err() {
                    // If an error has occurred, we are in a bad state, so prevent further iteration
                    self.0 = IterRepr::Empty;
                }
                result.transpose()
            }
            IterRepr::Slice(ref mut iter) => iter.next().cloned().map(|v| Ok(v)),
        }
    }
}

impl<'a, A, E> From<&'a [A]> for Repeated<'a, A, E>
where
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn from(value: &'a [A]) -> Self {
        Self::from_slice(value)
    }
}

#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
fn next_item<'a, A, E>(
    msg_buf: &mut list::MessageBuffer<'a>,
    packed_chunk: &mut &'a [u8],
) -> Result<Option<A>, error::DecodeError>
where
    A: 'a,
    E: item_encoding::ItemEncoding<'a, A>,
{
    if !packed_chunk.is_empty() {
        // We're partway through a packed occurrence; continue decoding elements from it.
        return Ok(Some(E::decode_single_value(packed_chunk)?));
    }

    let cursor = &mut msg_buf.data;
    while !cursor.is_empty() {
        let (tag, wire_type) = encoding::decode_key(cursor)?;
        if tag == msg_buf.tag {
            // At this point, we know for sure that this is a field tag occurrence that concerns
            // us, but which encoding/wire type was used?
            if wire_type == E::WIRE_TYPE {
                // Decode this single value (the unpacked case; also the only case for composite
                // element types such as messages/strings/bytes, whose wire type is length-delimited)
                return Ok(Some(E::decode_single_value(cursor)?));
            } else if wire_type == encoding::WireType::LengthDelimited {
                // A scalar repeated field encoded using the packed wire format. (`E::WIRE_TYPE`
                // is not length-delimited here, or the branch above would have matched.) Parse the
                // chunk and decode its first element; the remainder is kept for subsequent calls.
                let len = encoding::decode_varint(cursor)?;
                let len = usize::try_from(len)
                    .map_err(|_| error::DecodeError::LengthTooLargeForPlatform(len))?;
                if let Some((chunk, rest)) = cursor.split_at_checked(len) {
                    *cursor = rest;
                    if !chunk.is_empty() {
                        *packed_chunk = chunk;
                        return Ok(Some(E::decode_single_value(packed_chunk)?));
                    }
                    // An empty packed chunk contributes no elements; keep scanning.
                } else {
                    return Err(error::DecodeError::BufferUnderflow);
                }
            } else {
                return Err(error::DecodeError::UnexpectedWireTypeValue {
                    actual: wire_type,
                    expected: E::WIRE_TYPE,
                });
            }
        } else {
            encoding::skip_field(wire_type, tag, cursor)?;
        }
    }
    // We consumed the entire message buffer; there can't be any further occurrences
    Ok(None)
}
