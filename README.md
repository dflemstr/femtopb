# femtopb

A tiny footprint, `#[no_std]`, no-alloc, no-panic Protobuf serialization library.  This allows you to communicate using
Protobuf on constrained platforms, like bare-metal MCUs with very limited RAM.

Yes, you heard it right: this library lets you serialize and deserialize Protobuf messages without any dynamic
memory/heap allocation.

The library takes care of using simple types with limited use of generics when possible, to avoid monomorphization
code size explosion.  The runtime also consists of many tiny functions so that the ones that aren't used can get
optimized away.

During testing of this crate, checks are made to ensure that `femtopb` code cannot panic.  If  you want to leverage the
no-panic checks yourself to debug your own project, enable the `assert-no-panic` crate feature.  It is not necessarily a
good idea to enable this feature for your release code, as enabling this feature might change the generated code
slightly.

There are some limitations, however.  Messages must be deserialized from continuous `&[u8]` slices, so incremental
deserialization from other `bytes::Buf` types or streams/files/sockets/... is not supported.  Also, messages borrow
their source slice for the duration of their lifetime.  Deserialization of repeated fields happens lazily to avoid
having to dynamically allocate a `Vec`-like data structure.

The library does not implement advanced features like Protobuf reflection or well-known types, etc.  For that, use the
[`prost`][prost] crate.  It is probably advisable to use `prost` for applications where it is possible, and only use
`femtopb` on platforms where it is necessary.

## Attribution

The API is heavily inspired by the [`prost`][prost] crate, which is licensed under the Apache 2.0 license.  Some tests
and key algorithms have also been copied from that crate. This crate is also licensed under the Apache 2.0 license.

[prost]: https://crates.io/crates/prost
