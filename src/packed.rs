//! `Packed` scalar values.
use core::{fmt, marker, slice};

use crate::error;
use crate::list;
use crate::window;
use crate::{encoding, item_encoding};

/// A tightly-packed encoding of a sequence of scalar values.
///
/// Use `.iter()` to iterate through the elements of this `Packed`.
///
/// The `window` is stored flat here, alongside the `list`, rather than inside `list`'s
/// `MessageBuffer` variant: growing it while decoding is then an unconditional write, which keeps
/// the decode path provably panic-free (see [`window::Window`]).
pub struct Packed<'a, A, E>(
    list::List<'a, A>,
    window::Window<'a>,
    marker::PhantomData<E>,
)
where
    E: item_encoding::ItemEncoding<'a, A>;

/// An iterator for a `Packed`.
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
        packed_chunk: &'a [u8],
        phantom: marker::PhantomData<E>,
    },
    Slice(slice::Iter<'a, A>),
}

impl<'a, A, E> Packed<'a, A, E>
where
    E: item_encoding::ItemEncoding<'a, A>,
{
    /// Creates a new, empty `Packed` with minimal memory footprint.
    #[must_use]
    pub const fn empty() -> Self {
        Self(
            list::List::empty(),
            window::Window::empty(),
            marker::PhantomData,
        )
    }

    /// Creates a `Packed` that uses the specified slice as its storage.
    ///
    /// The slice must live as long as this `Packed` does.
    #[must_use]
    pub fn from_slice(slice: &'a [A]) -> Self {
        Self(
            list::List::from_slice(slice),
            window::Window::empty(),
            marker::PhantomData,
        )
    }

    // Used internally by the runtime during decoding. `msg_buf` is the whole message buffer and
    // `field_start` (a suffix of it) is where the field's first occurrence begins; the retained
    // window initially covers from there to the end of the buffer (so an un-narrowed field still
    // re-scans correctly) and `extend_region` narrows its tail to end at the last occurrence.
    pub(crate) fn from_msg_buf(tag: u32, msg_buf: &'a [u8], field_start: &[u8]) -> Self {
        Self(
            list::List::from_msg_buf(tag),
            window::Window::covering(msg_buf, field_start),
            marker::PhantomData,
        )
    }

    // Used internally by the runtime during decoding, to grow the retained window so it ends at the
    // occurrence that was just skipped. This is a flat, unconditional write (see the note on the
    // struct), so it stays off the optimizer's critical path for the no-panic proof.
    #[inline]
    pub(crate) fn extend_region(&mut self, remaining: &[u8]) {
        self.1.extend_to(remaining);
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

impl<'a, A, E> Packed<'a, A, E>
where
    A: Copy,
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
        self.iter().next().is_none()
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
    fn from_parts(lst: list::List<'a, A>, window: window::Window<'a>) -> Self {
        let repr = match lst {
            list::List::Empty => IterRepr::Empty,
            // Iterate over the narrowed window only; hand the cursor exactly the field's span.
            list::List::MessageBuffer(tag) => IterRepr::MessageBuffer {
                msg_buf: list::MessageBuffer {
                    tag,
                    data: window.region(),
                },
                packed_chunk: &[],
                phantom: marker::PhantomData,
            },
            list::List::Slice(slice) => IterRepr::Slice(slice.iter()),
        };
        Self(repr)
    }
}

impl<'a, A, E> PartialEq for Packed<'a, A, E>
where
    A: Copy + PartialEq,
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

// Implemented manually since deriving `Clone` would wrongly require `A: Clone`; `Packed` is `Copy`
// regardless of `A`, so a plain copy suffices.
impl<'a, A, E> Clone for Packed<'a, A, E>
where
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, A, E> Copy for Packed<'a, A, E> where E: item_encoding::ItemEncoding<'a, A> {}

impl<'a, A, E> Default for Packed<'a, A, E>
where
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, A, E> fmt::Debug for Packed<'a, A, E>
where
    A: Copy + fmt::Debug,
    E: item_encoding::ItemEncoding<'a, A>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list_fmt = f.debug_list();
        for ref item in self.iter() {
            match item {
                Ok(item) => {
                    list_fmt.entry(item);
                }
                Err(e) => {
                    list_fmt.entry(&format_args!("...error: {:?}", e));
                    break;
                }
            }
        }
        list_fmt.finish()
    }
}

#[cfg(feature = "defmt")]
impl<'a, A, E> defmt::Format for Packed<'a, A, E>
where
    A: Copy + defmt::Format,
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

impl<'a, A, E> IntoIterator for Packed<'a, A, E>
where
    A: Copy,
    E: item_encoding::ItemEncoding<'a, A>,
{
    type Item = Result<A, error::DecodeError>;
    type IntoIter = Iter<'a, A, E>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::from_parts(self.0, self.1)
    }
}

impl<'a, A, E> IntoIterator for &Packed<'a, A, E>
where
    A: Copy,
    E: item_encoding::ItemEncoding<'a, A>,
{
    type Item = Result<A, error::DecodeError>;
    type IntoIter = Iter<'a, A, E>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::from_parts(self.0, self.1)
    }
}

impl<'a, A, E> Iterator for Iter<'a, A, E>
where
    A: Copy,
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
            IterRepr::Slice(ref mut iter) => iter.next().map(|v| Ok(*v)),
        }
    }
}

impl<'a, A, E> From<&'a [A]> for Packed<'a, A, E>
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
    if packed_chunk.is_empty() {
        // Need to "fetch" a new chunk; try to find next occurrence of our tag.
        // By taking a mut reference to msg_buf.data, we ensure that the original slice gradually
        // gets updated, so that msg_buf.data only contains what remains to be parsed from the
        // overall message buffer
        let cursor = &mut msg_buf.data;
        while !cursor.is_empty() {
            let (tag, wire_type) = encoding::decode_key(cursor)?;
            if tag == msg_buf.tag {
                // At this point, we know for sure that this is a field tag occurrence that concerns
                // us, but which encoding/wire type was used?
                if wire_type == encoding::WireType::LengthDelimited {
                    let len = encoding::decode_varint(cursor)?;
                    let len = usize::try_from(len)
                        .map_err(|_| error::DecodeError::LengthTooLargeForPlatform(len))?;
                    // It would be odd to get a packed field with length zero, but technically
                    // possible...
                    if len > 0 {
                        // At this point, packed_chunk is a mut reference to wherever the original
                        // slice is stored (most likely a list::Iter::MessageBuffer) which means
                        // that the remainder of this chunk will be used on the next call to
                        // Iter::next(). `split_at_checked` (rather than the panicking `split_at`)
                        // keeps this iteration panic-free — matching `Repeated` — if the declared
                        // packed length runs past the retained buffer.
                        let Some((chunk, rest)) = cursor.split_at_checked(len) else {
                            return Err(error::DecodeError::BufferUnderflow);
                        };
                        *packed_chunk = chunk;
                        *cursor = rest;
                        return Ok(Some(E::decode_single_value(packed_chunk)?));
                    }
                    // fall through until next loop iteration here; we don't need to
                    // cursor.advance() since the len was 0
                } else if wire_type == E::WIRE_TYPE {
                    // We can handle the case where a packed field is actually encoded as a repeated
                    // field
                    return Ok(Some(E::decode_single_value(cursor)?));
                } else {
                    return Err(error::DecodeError::UnexpectedWireTypeValue {
                        actual: wire_type,
                        expected: encoding::WireType::LengthDelimited,
                    });
                }
            } else {
                encoding::skip_field(wire_type, tag, cursor)?;
            }
        }
        // We consumed the entire message buffer; there can't be any further occurrences
        Ok(None)
    } else {
        // The packed_chunk contains data.  We now try to parse that data or die trying.  Note that
        // it is not considered valid for a packed chunk to contain garbage underflow data at the
        // end, which is why we treat that scenario as an irrecoverable error here.
        Ok(Some(E::decode_single_value(packed_chunk)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_empty() {
        let packed: Packed<i32, item_encoding::Int32> = Packed::empty();
        assert!(packed.is_empty());
        assert!(packed.is_unpopulated());
        assert_eq!(packed.len(), 0);
        assert_eq!(packed.iter().collect::<Vec<_>>().as_slice(), &[]);
    }

    #[test]
    fn packed_empty_slice() {
        let packed: Packed<i32, item_encoding::Int32> = Packed::from_slice(&[]);
        assert!(packed.is_empty());
        assert!(!packed.is_unpopulated());
        assert_eq!(packed.len(), 0);
        assert_eq!(packed.iter().collect::<Vec<_>>().as_slice(), &[]);
    }

    #[test]
    fn packed_nonempty_slice() {
        let packed: Packed<i32, item_encoding::Int32> = Packed::from_slice(&[1, 2, 3]);
        assert!(!packed.is_empty());
        assert!(!packed.is_unpopulated());
        assert_eq!(packed.len(), 3);
        assert_eq!(
            packed.iter().collect::<Vec<_>>().as_slice(),
            &[Ok(1), Ok(2), Ok(3)]
        );
    }

    #[test]
    fn packed_length_running_past_buffer_errors_without_panicking() {
        // A packed field whose length prefix (5) claims more bytes than remain (2). Decoding this
        // used to hit `split_at`, which panics; it must now surface a clean `BufferUnderflow`
        // instead (matching `Repeated`), keeping iteration panic-free on hostile input.
        let tag = 1;
        let key = encoding::WireType::LengthDelimited as u8 | tag << 3;
        let msgbuf = &[key, 0x05, 0x01, 0x02];
        let packed: Packed<i32, item_encoding::Int32> =
            Packed::from_msg_buf(tag as u32, msgbuf, msgbuf);
        assert_eq!(
            packed.iter().collect::<Vec<_>>().as_slice(),
            &[Err(error::DecodeError::BufferUnderflow)]
        );
    }

    #[test]
    fn packed_empty_msgbuf() {
        let tag = 1;
        let key = encoding::WireType::LengthDelimited as u8 | tag << 3;
        let len = 0;
        let msgbuf = &[key, len];
        let packed: Packed<i32, item_encoding::Int32> =
            Packed::from_msg_buf(tag as u32, msgbuf, msgbuf);
        assert!(packed.is_empty());
        assert!(!packed.is_unpopulated());
        assert_eq!(packed.len(), 0);
        assert_eq!(packed.iter().collect::<Vec<_>>().as_slice(), &[]);
    }

    #[test]
    fn packed_empty_msgbuf_ignore_other_tags() {
        let tag = 1;
        let other_key_1 = encoding::WireType::LengthDelimited as u8 | 2 << 3;
        let other_key_2 = encoding::WireType::LengthDelimited as u8 | 3 << 3;
        let other_key_3 = encoding::WireType::ThirtyTwoBit as u8 | 4 << 3;
        let other_key_4 = encoding::WireType::ThirtyTwoBit as u8 | 5 << 3;
        let key = encoding::WireType::LengthDelimited as u8 | tag << 3;
        let len = 0;
        let msgbuf = &[
            other_key_1,
            0,
            other_key_3,
            0,
            0,
            0,
            0,
            key,
            len,
            other_key_2,
            0,
            other_key_4,
            0,
            0,
            0,
            0,
        ];
        let packed: Packed<i32, item_encoding::Int32> =
            Packed::from_msg_buf(tag as u32, msgbuf, msgbuf);
        assert!(packed.is_empty());
        assert!(!packed.is_unpopulated());
        assert_eq!(packed.len(), 0);
        assert_eq!(packed.iter().collect::<Vec<_>>().as_slice(), &[]);
    }

    #[test]
    fn packed_nonempty_msgbuf() {
        let tag = 1;
        let key = encoding::WireType::LengthDelimited as u8 | tag << 3;
        let len = 3;
        let msgbuf = &[key, len, 1, 2, 3];
        let packed: Packed<i32, item_encoding::Int32> =
            Packed::from_msg_buf(tag as u32, msgbuf, msgbuf);
        assert!(!packed.is_empty());
        assert!(!packed.is_unpopulated());
        assert_eq!(packed.len(), 3);
        assert_eq!(
            packed.iter().collect::<Vec<_>>().as_slice(),
            &[Ok(1), Ok(2), Ok(3)]
        );
    }

    #[test]
    fn packed_nonempty_msgbuf_ignore_other_tags() {
        let tag = 1;
        let other_key_1 = encoding::WireType::LengthDelimited as u8 | 2 << 3;
        let other_key_2 = encoding::WireType::LengthDelimited as u8 | 3 << 3;
        let other_key_3 = encoding::WireType::ThirtyTwoBit as u8 | 4 << 3;
        let other_key_4 = encoding::WireType::ThirtyTwoBit as u8 | 5 << 3;

        let key = encoding::WireType::LengthDelimited as u8 | tag << 3;
        let len = 3;
        let msgbuf = &[
            other_key_1,
            0,
            other_key_3,
            0,
            0,
            0,
            0,
            key,
            len,
            1,
            2,
            3,
            other_key_2,
            0,
            other_key_4,
            0,
            0,
            0,
            0,
        ];
        let packed: Packed<i32, item_encoding::Int32> =
            Packed::from_msg_buf(tag as u32, msgbuf, msgbuf);
        assert!(!packed.is_empty());
        assert!(!packed.is_unpopulated());
        assert_eq!(packed.len(), 3);
        assert_eq!(
            packed.iter().collect::<Vec<_>>().as_slice(),
            &[Ok(1), Ok(2), Ok(3)]
        );
    }
}
