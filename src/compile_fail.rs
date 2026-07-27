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
