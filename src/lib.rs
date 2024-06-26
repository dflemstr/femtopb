#![cfg_attr(not(test), no_std)]
//! # `femtopb`
//!
//! A tiny footprint, `#[no_std]`, no-`alloc`, no-panic Protobuf serialization library.  This allows
//! you to communicate using Protobuf on constrained platforms, like bare-metal MCUs with very
//! limited RAM.
//!
//! Yes, you heard it right: this library lets you serialize and deserialize Protobuf messages
//! without any dynamic memory/heap allocation.
//!
//! The library takes care of using simple types with limited use of generics when possible, to
//! avoid monomorphization code size explosion.  The runtime also consists of many tiny functions
//! so that the ones that aren't used can get optimized away.
//!
//! During testing of this crate, checks are made to ensure that `femtopb` code cannot panic.  If
//! you want to leverage the no-panic checks yourself to debug your own project, enable the
//! `assert-no-panic` crate feature.  It is not necessarily a good idea to enable this feature for
//! your release code, as enabling this feature might change the generated code slightly.
//!
//! ## Defining message types
//!
//! `femtopb` enables encoding and decoding of messages by deriving the `femtopb::Message` trait.
//! Writing the code for these message types is usually done using the `femtopb-build` crate, but
//! we will write some code manually here for illustration purposes.
//!
//! A simple message type might look like this:
//!
//! ```
//! #[derive(Clone, femtopb::Message)]
//! pub struct Person<'a> {
//!     #[femtopb(uint32, tag = 1)]
//!     pub age: u32,
//!     #[femtopb(string, tag = 2)]
//!     pub name: &'a str,
//!     #[femtopb(unknown_fields)]
//!     pub unknown_fields: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! The struct can be used just like any other Rust struct, using the simple built-in types of the
//! language for the most part.
//!
//! The lifetime parameter on the struct is mandatory, and is used to refer to the lifetime of any
//! dynamically sized data inside the message.  When a message is decoded from raw bytes, types like
//! `&'a str` will borrow its contents from the original raw byte buffer.
//!
//! The `#[femtopb(...)]` attribute provides additional metadata for how the type gets serialized.
//! All the semantics are described in the [official protobuf docs](https://protobuf.dev/programming-guides/encoding/),
//! but as a brief overview, the `tag` number corresponds to the binary ID of the field as it is
//! serialized on the wire (and should hence be unique, and not change, for your given message type).
//! Other bits in the attribute influence how the data gets serialized more specifically; again,
//! please consult the official protobuf documentation for details.
//!
//! The `#[femtopb(unknown_fields)]` field is used to preserve fields that are not yet known to our
//! current code.  For example, if the writer of a message has added a new field to the message
//! (which is a backwards-compatible change), the data of this field will be preserved inside the
//! `unknown_fields` field.  The API for this is still a work in progress.
//!
//! ## Encoding and decoding
//!
//! Given a message definition like the one above, you can easily encode and decode data to/from
//! bytes, using the associated trait methods:
//!
//! ```
//! use femtopb::Message as _;
//!
//! // Adding some more derived traits to aid with the example below:
//! #[derive(Clone, Debug, PartialEq, femtopb::Message)]
//! pub struct Person<'a> {
//!     #[femtopb(uint32, tag = 1)]
//!     pub age: u32,
//!     #[femtopb(string, tag = 2)]
//!     pub name: &'a str,
//!     #[femtopb(unknown_fields)]
//!     pub unknown_fields: femtopb::UnknownFields<'a>,
//! }
//!
//! fn main() {
//!     let person = Person { age: 32, name: "David", ..Default::default() };
//!     // Create a new buffer filled with zeroes.  This of course doesn't need to be dynamically
//!     // allocated; at this point, you could use a stack-allocated buffer, or `'static` memory
//!     // region, for example.  Here, we use a `Vec` for simplicity.
//!     // The buffer MUST already have the right length, since we can't grow the buffer without
//!     // dynamic memory allocation.
//!     let mut buf = vec![0; person.encoded_len()];
//!
//!     // Encode the Person to the buffer
//!     person.encode(&mut buf.as_mut_slice()).unwrap();
//!     // Decode a new Person from the same buffer
//!     let new_person = Person::decode(buf.as_slice()).unwrap();
//!
//!     // The same information should be preserved!
//!     assert_eq!(person, new_person);
//! }
//! ```
//!
//! ## Repeated and packed fields
//!
//! Quite commonly, you'll need to encode a collection of things.  In `femtopb`, since we can't
//! do any dynamic memory allocation, we need to use special collection types that borrow all of
//! their memory from the original buffer, instead of types like `Vec` that use dynamic allocation.
//!
//! These types decode their values on-the-fly, when you iterate through them, instead of eagerly
//! at `decode()` time.  Hence, they also need to know about an `ItemEncoding`, which is used to
//! decode values lazily after the main `Message::decode()` call has returned.  This means that the
//! type is quite long, but you usually won't see it in code generated by `femtopb-build`.
//!
//! There are repeated fields, which are usually used for composite types like messages, and packed
//! fields, which use a more efficient encoding for scalar types.  You usually don't have to make
//! an active choice as to which type to use; `femtopb-build` will generate the right type for you.
//!
//! ```
//! use femtopb::Message as _;
//! use femtopb::item_encoding;
//! use femtopb::repeated;
//!
//! #[derive(Clone, Debug, PartialEq, femtopb::Message)]
//! pub struct WeatherStationEvent<'a> {
//!     #[femtopb(bytes, tag = 1)]
//!     serial_id: &'a [u8],
//!     #[femtopb(message, repeated, tag = 2)]
//!     new_readings: repeated::Repeated<'a, TempReading<'a>, item_encoding::Message<'a, TempReading<'a>>>,
//!     #[femtopb(unknown_fields)]
//!     pub unknown_fields: femtopb::UnknownFields<'a>,
//! }
//!
//! #[derive(Clone, Debug, PartialEq, femtopb::Message)]
//! pub struct TempReading<'a> {
//!     #[femtopb(uint64, tag = 1)]
//!     id: u64,
//!     #[femtopb(float, tag = 2)]
//!     degrees_c: f32,
//!     #[femtopb(float, tag = 3)]
//!     pressure_hpa: f32,
//!     #[femtopb(unknown_fields)]
//!     pub unknown_fields: femtopb::UnknownFields<'a>,
//! }
//!
//! fn main() {
//!     let new_readings = &[
//!         TempReading { id: 1234, degrees_c: 23.0, ..Default::default() },
//!         TempReading { id: 1235, degrees_c: 23.2, ..Default::default() },
//!         TempReading { id: 1236, degrees_c: 23.4, ..Default::default() },
//!     ];
//!     let event = WeatherStationEvent {
//!         serial_id: b"mystation-abc123",
//!         new_readings: repeated::Repeated::from_slice(new_readings),
//!         ..Default::default()
//!     };
//!
//!     let mut buf = vec![0; event.encoded_len()];
//!     event.encode(&mut buf.as_mut_slice()).unwrap();
//!     // ... send buf over a LoRa network or something ...
//!     let new_event = WeatherStationEvent::decode(buf.as_slice()).unwrap();
//!     assert_eq!(new_event.serial_id, b"mystation-abc123");
//!     for reading in &new_event.new_readings {
//!         let TempReading { id, degrees_c, .. } = reading.unwrap();
//!         println!("{id}: {degrees_c}");
//!         // Prints:
//!         // 1234: 23.0
//!         // 1235: 23.2
//!         // 1236: 23.4
//!     }
//! }
//! ```
//!
//! ## Enums
//!
//! Enums are represented via the `femtopb::Enumeration` trait.  The only special requirements on
//! enums are that the enum must derive `Clone` and `Copy`, and all variants must have a
//! discriminant (number value) assigned to them.  Also, generated enums will always have the first
//! variant be its default value.
//!
//! Enums cannot themselves be encoded; they must exist as a field on a `Message`, and then the
//! wrapping message can be encoded.
//!
//! ```
//! #[derive(Clone, Copy, Default, femtopb::Enumeration)]
//! pub enum BasicEnumeration {
//!     #[default]
//!     ZERO = 0,
//!     ONE = 1,
//!     TWO = 2,
//!     THREE = 3,
//! }
//! ```
//!
//! ## Oneofs
//!
//! Oneofs are enums where each variant wraps exactly one other value.  The `oneof` terminology
//! comes from the related [protobuf concept](https://protobuf.dev/programming-guides/proto3/#oneof).
//!
//! Oneofs can be used to encode mutually exclusive values.  On the wire, oneofs are simply encoded
//! as the field out of a group of mutually exclusive fields that was actually populated.
//!
//! Like with enums, oneofs can't be encoded on their own, and must exist as a field on a message.
//!
//! ```
//! #[derive(femtopb::Oneof)]
//! pub enum StorePageReview<'a> {
//!     #[femtopb(int32, tag = 8)]
//!     Rating(i32),
//!     #[femtopb(string, tag = 9)]
//!     WrittenReview(&'a str),
//! }
//! ```
//!
//! ## Other features
//!
//! There are simpler concepts like optional fields, recursive messages, etc. that are not yet
//! covered in this documentation.  Feel free to request documentation for anything that you feel
//! is missing!
//!
//! ## Unsupported protobuf features
//!
//! This library does not (yet) support groups (which were deprecated since Protobuf 1) and maps.
//!
//! ## Acknowledgements
//!
//! This library is heavily inspired by the amazing `prost` library, and some tests and core
//! algorithms were copied from that library.  However, the architecture of `femtopb` compared to
//! `prost` ended up being significantly different, so mostly only the build infrastructure of
//! `prost-build` was re-used in the creation of `femtopb-build`.

mod bits;
mod list;

pub mod deferred;
pub mod encoding;
pub mod enumeration;
pub mod error;
pub mod item_encoding;
pub mod message;
pub mod oneof;
pub mod packed;
pub mod repeated;
pub mod unknown_fields;

#[doc(hidden)]
pub mod runtime;

pub use enumeration::EnumValue;
pub use enumeration::Enumeration;
pub use femtopb_derive::Enumeration;
pub use femtopb_derive::Message;
pub use femtopb_derive::Oneof;
pub use message::Message;
pub use oneof::Oneof;
pub use packed::Packed;
pub use repeated::Repeated;
pub use unknown_fields::UnknownFields;
