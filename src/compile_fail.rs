//! Compile-fail coverage for the derive macros' error reporting.
//!
//! The macros reject a range of malformed schemas with a `compile_error!`. These `compile_fail`
//! doctests pin that rejection: each snippet is otherwise well-formed, so the *only* reason it fails
//! to compile is the mistake it demonstrates. The module is gated on `cfg(doctest)` so it costs
//! nothing in a normal build.
//!
//! Baseline — a well-formed message derives cleanly, proving the setup compiles:
//!
//! ```rust
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct Ok<'a> {
//!     #[femtopb(int32, tag = 1)]
//!     n: i32,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! `Message` cannot be derived for an enum:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! enum E<'a> {
//!     A(&'a str),
//! }
//! ```
//!
//! A field may not specify both `tag` and `tags`:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(int32, tag = 1, tags = [1, 2])]
//!     n: i32,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! A field may not declare two scalar types:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(int32, int64, tag = 1)]
//!     n: i32,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! `packed` and `optional` are mutually exclusive:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(int32, packed, optional, tag = 1)]
//!     xs: femtopb::Packed<'a, i32, femtopb::item_encoding::Int32>,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! An unrecognised attribute argument is rejected:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(int32, nonsense, tag = 1)]
//!     n: i32,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! `group` fields are unsupported:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(group, tag = 1)]
//!     g: i32,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! `map` fields are unsupported:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(map, tag = 1)]
//!     m: i32,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! `Enumeration` cannot be derived for a struct:
//!
//! ```compile_fail
//! #[derive(Clone, Copy, Default, femtopb::Enumeration)]
//! struct S;
//! ```
//!
//! Every `Enumeration` variant needs an explicit discriminant:
//!
//! ```compile_fail
//! #[derive(Clone, Copy, Default, femtopb::Enumeration)]
//! enum E {
//!     #[default]
//!     A,
//! }
//! ```
//!
//! `Oneof` cannot be derived for a struct:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Oneof)]
//! struct S;
//! ```
//!
//! An `Oneof` variant must have exactly one field — zero is rejected:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Oneof)]
//! enum E {
//!     #[femtopb(int32, tag = 1)]
//!     A,
//! }
//! ```
//!
//! — and more than one is rejected:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Oneof)]
//! enum E<'a> {
//!     #[femtopb(int32, tag = 1)]
//!     A(i32, &'a str),
//! }
//! ```
//!
//! A tag below the protobuf minimum of 1 is rejected at expansion time (encoding it would write a
//! key the decoder rejects):
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(int32, tag = 0)]
//!     n: i32,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! — as is one above the maximum of 2^29 - 1, which would not fit in the 29 bits a field key
//! reserves for the tag:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(int32, tag = 536870912)]
//!     n: i32,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! The same bounds apply to every entry of a `tags` list:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(oneof, tags = [1, 536870912])]
//!     c: Option<C>,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//!
//! #[derive(Clone, PartialEq, femtopb::Oneof)]
//! enum C {
//!     #[femtopb(int32, tag = 1)]
//!     N(i32),
//! }
//! ```
//!
//! Two fields of a message may not share a tag; the second would never decode:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(int32, tag = 1)]
//!     a: i32,
//!     #[femtopb(int32, tag = 1)]
//!     b: i32,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//! ```
//!
//! — nor may a plain field collide with one of a oneof's tags:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(int32, tag = 3)]
//!     a: i32,
//!     #[femtopb(oneof, tags = [3, 4])]
//!     c: Option<C>,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//!
//! #[derive(Clone, PartialEq, femtopb::Oneof)]
//! enum C {
//!     #[femtopb(int32, tag = 3)]
//!     N(i32),
//! }
//! ```
//!
//! Two variants of a oneof may not share a tag either:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Oneof)]
//! enum C {
//!     #[femtopb(int32, tag = 8)]
//!     N(i32),
//!     #[femtopb(int64, tag = 8)]
//!     M(i64),
//! }
//! ```
//!
//! A `oneof` field must list its variants' tags, or the generated decoder would have nothing to
//! match on and the field would silently never decode:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(oneof)]
//!     c: Option<C>,
//!     #[femtopb(unknown_fields)]
//!     u: femtopb::UnknownFields<'a>,
//! }
//!
//! #[derive(Clone, PartialEq, femtopb::Oneof)]
//! enum C {
//!     #[femtopb(int32, tag = 1)]
//!     N(i32),
//! }
//! ```
//!
//! A message may declare at most one `unknown_fields` field, since only the first backs the
//! decoder's catch-all arm:
//!
//! ```compile_fail
//! #[derive(Clone, PartialEq, femtopb::Message)]
//! struct S<'a> {
//!     #[femtopb(int32, tag = 1)]
//!     n: i32,
//!     #[femtopb(unknown_fields)]
//!     u1: femtopb::UnknownFields<'a>,
//!     #[femtopb(unknown_fields)]
//!     u2: femtopb::UnknownFields<'a>,
//! }
//! ```
