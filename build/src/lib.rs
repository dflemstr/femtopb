//! # `femtopb-build`
//!
//! A code generator/protobuf compiler for the `femtopb` library.  Uses `protox` and `prost-build`
//! under the hood, but for now only a limited subset of their full APIs are exposed.
//!
//! ## Usage
//!
//! This library is meant to be used in your `build.rs` script to generate Rust code at build time.
//! First, add both `femtopb` and `femtopb-build` to your dependencies like this:
//!
//! ```toml
//! [dependencies]
#![doc=concat!("femtopb = \"", env!("CARGO_PKG_VERSION"), "\"")]
//!
//! [build-dependencies]
#![doc=concat!("femtopb-build = \"", env!("CARGO_PKG_VERSION"), "\"")]
//! ```
//! An example of a valid `build.rs` file is:
//!
//! ```rust,ignore
//! fn main() -> anyhow::Result<()> {
//!     femtopb_build::compile_protos(
//!         &["src/myapi/v1/myapi.proto", "src/myapi/v1/foo.proto"],
//!         &["src"],
//!     )
//! }
//! ```
//!
//! The first argument to `compile_protos` lists which proto schema files to compile,
//! and the second argument lists include dirs, where imports from one proto file to another
//! will get resolved.
//!
//! You may then include the parts of the schema that you want to use in your application. The file
//! name of the generated file will be based on the protobuf `package` declaration (and for sanity
//! should probably match your directory structure, too).
//!
//! ```rust,ignore
//! pub mod myapi {
//!     pub mod v1 {
//!         include!(concat!(env!("OUT_DIR"), "/myapi.v1.rs"));
//!     }
//! }
//!
//! use myapi::v1::Foo;
//! // ...
//! ```
//!
//! To view the generated code, the easiest way is probably to just run `cargo doc`.
//!
//! ## Checking in generated code
//!
//! If you don't want to generate the code during the build, another common approach is to generate
//! the code once and check in the generated code in source control.  A common, but hacky, way is to
//! add an `example` to your crate that generates the code.
//!
//! For example, create a file called `examples/mycrate-generate-schema.rs` containing:
//!
//! ```rust,ignore
//! fn main() -> anyhow::Result<()> {
//!     femtopb_build::compile_protos_into(
//!         &["src/myapi/v1/myapi.proto", "src/myapi/v1/foo.proto"],
//!         &["src"],
//!         "src",
//!     )
//! }
//! ```
//!
//! Here, we use the `compile_protos_into` function that lets you specify a custom output directory,
//! and we use the `src` dir of the crate to have the schemas live next to the rest of the
//! application code (you may of course decide to structure things differently).

use std::collections;
use std::env;
use std::fs;
use std::path;

// This API is intentionally somewhat limited.  It might make sense to add more functions to it
// eventually... however, we probably don't want the full customizability of `prost` here.

/// Compile `.proto` files into Rust files during a Cargo build.
///
/// The generated `.rs` files are written to the Cargo `OUT_DIR` directory, suitable for use with
/// the [include!][1] macro. See the [Cargo `build.rs` code generation][2] example for more info.
///
/// This function should be called in a project's `build.rs`.
///
/// # Arguments
///
/// **`protos`** - Paths to `.proto` files to compile. Any transitively [imported][3] `.proto`
/// files are automatically be included.
///
/// **`includes`** - Paths to directories in which to search for imports. Directories are searched
/// in order. The `.proto` files passed in **`protos`** must be found in one of the provided
/// include directories.
///
/// # Errors
///
/// This function can fail for a number of reasons:
///
///   - Failure to parse the `.proto`s.
///   - Failure to locate an imported `.proto`.
///   - Failure to compile a `.proto` without a [package specifier][4].
///
/// It's expected that this function call be `unwrap`ed in a `build.rs`; there is typically no
/// reason to gracefully recover from errors during a build.
///
/// # Example `build.rs`
///
/// ```rust,no_run
/// # use std::io::Result;
/// fn main() -> anyhow::Result<()> {
///     femtopb_build::compile_protos(&["src/frontend.proto", "src/backend.proto"], &["src"])
/// }
/// ```
///
/// [1]: https://doc.rust-lang.org/std/macro.include.html
/// [2]: http://doc.crates.io/build-script.html#case-study-code-generation
/// [3]: https://developers.google.com/protocol-buffers/docs/proto3#importing-definitions
/// [4]: https://developers.google.com/protocol-buffers/docs/proto#packages
pub fn compile_protos(
    protos: &[impl AsRef<path::Path>],
    includes: &[impl AsRef<path::Path>],
) -> anyhow::Result<()> {
    Config::new().protos(protos).includes(includes).compile()
}

/// Like `compile_protos`, but lets you specify the target directory explicitly, instead of relying
/// on the convention used by cargo of using the `OUT_DIR` env var.
pub fn compile_protos_into(
    protos: &[impl AsRef<path::Path>],
    includes: &[impl AsRef<path::Path>],
    target: impl AsRef<path::Path>,
) -> anyhow::Result<()> {
    Config::new()
        .target(target)
        .protos(protos)
        .includes(includes)
        .compile()
}

pub struct Config {
    protos: Vec<path::PathBuf>,
    includes: Vec<path::PathBuf>,
    target: Option<path::PathBuf>,
    derive_defmt: bool,
}

#[derive(Default)]
struct FieldMetadata {
    is_scalar: Option<String>,
    is_message: bool,
    is_enum: Option<syn::Path>,
    is_oneof: Option<syn::Path>,
    is_repeated: bool,
    is_packed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            protos: Vec::new(),
            includes: Vec::new(),
            target: None,
            derive_defmt: false,
        }
    }

    pub fn target(&mut self, target: impl AsRef<path::Path>) -> &mut Self {
        self.target = Some(target.as_ref().to_owned());
        self
    }

    pub fn protos(&mut self, protos: &[impl AsRef<path::Path>]) -> &mut Self {
        self.protos = protos.iter().map(|p| p.as_ref().to_owned()).collect();
        self
    }

    pub fn includes(&mut self, includes: &[impl AsRef<path::Path>]) -> &mut Self {
        self.includes = includes.iter().map(|p| p.as_ref().to_owned()).collect();
        self
    }

    pub fn derive_defmt(&mut self, value: bool) -> &mut Self {
        self.derive_defmt = value;
        self
    }

    pub fn compile(&mut self) -> anyhow::Result<()> {
        let fds = protox::compile(&self.protos, &self.includes)?;
        let target = if let Some(ref t) = self.target {
            t.clone()
        } else {
            path::Path::new(
                &env::var_os("OUT_DIR")
                    .ok_or_else(|| anyhow::anyhow!("OUT_DIR environment variable is not set"))?,
            )
            .to_owned()
        };

        let requests = fds
            .file
            .into_iter()
            .map(|descriptor| {
                (
                    prost_build::Module::from_protobuf_package_name(descriptor.package()),
                    descriptor,
                )
            })
            .collect::<Vec<_>>();

        let file_names = requests
            .iter()
            .map(|req| (req.0.clone(), req.0.to_file_name_or("_")))
            .collect::<collections::HashMap<prost_build::Module, String>>();

        let mut config = prost_build::Config::new();
        config.format(false).bytes(["."]).prost_path("::femtopb");

        if self.derive_defmt {
            config.message_attribute(".", r#"#[derive(::defmt::Format)]"#);
            config.enum_attribute(".", r#"#[derive(::defmt::Format)]"#);
        }

        let modules = config.generate(requests)?;

        for (module, content) in modules.into_iter() {
            let content = transform(&content);

            let file_name = file_names
                .get(&module)
                .expect("every module should have a filename");
            let output_path = target.join(file_name);

            let previous_content = fs::read(&output_path);

            if previous_content
                .map(|previous_content| previous_content == content.as_bytes())
                .unwrap_or(false)
            {
                tracing::trace!("unchanged: {:?}", file_name);
            } else {
                tracing::trace!("writing: {:?}", file_name);
                fs::write(output_path, content)?;
            }
        }

        Ok(())
    }
}

// Here begins the adventure of converting prost-build output into the equivalent femtopb code.
// It would of course be great if there was a more clean API for this, e.g. if prost-build would
// expose a codegen API.  For now though, this somewhat hacky code will have to do.  I haven't
// been following quite as high of a quality bar in this code compared to the other code in this
// workspace, since this code is only ever ran at build time, and hence doesn't need to be super
// optimized.

fn transform(content: &str) -> String {
    let mut file = syn::parse_file(content).unwrap();

    // Oneofs made up solely of scalar variants must not carry a lifetime (an unused lifetime is a
    // hard error), so they are emitted without one and without the `PhantomData` variant. Collect
    // their names up front, since a message field can reference a oneof declared later in the file.
    let scalar_only_oneofs = collect_scalar_only_oneofs(&file.items);

    // Generated code is machine output, not hand-written idiomatic Rust, and its exact shape can
    // shift with the prost/protobuf toolchain. Suppress lints on every generated item so the code
    // stays warning-clean. `clippy::all` covers e.g. oneofs, which are legitimately large-variant
    // enums that cannot be boxed in a no-alloc crate; `deprecated` covers the encode/decode code the
    // derive generates for a message's own `#[deprecated]` fields, which it must read to round-trip
    // them. This is applied as an *outer* attribute on each top-level item (a module attribute
    // covers everything nested inside it) rather than as a module-level *inner* attribute, so the
    // output can be pulled in with `include!(...)` inside a `mod { ... }` block — which rejects a
    // leading inner attribute — as well as via the usual `pub mod foo;` file wiring.
    let allow: syn::Attribute = syn::parse_quote!(#[allow(clippy::all, deprecated)]);
    for item in &mut file.items {
        transform_item(item, &scalar_only_oneofs);
        if let Some(attrs) = item_attrs_mut(item) {
            attrs.insert(0, allow.clone());
        }
    }

    prettyplease::unparse(&file)
}

/// The attribute list of a top-level item, for the kinds of items generated code contains
/// (messages become structs, enums/oneofs become enums, `as_str_name` etc. become impls, nested
/// messages become modules). Returns `None` for item kinds that cannot carry attributes.
fn item_attrs_mut(item: &mut syn::Item) -> Option<&mut Vec<syn::Attribute>> {
    match item {
        syn::Item::Struct(i) => Some(&mut i.attrs),
        syn::Item::Enum(i) => Some(&mut i.attrs),
        syn::Item::Impl(i) => Some(&mut i.attrs),
        syn::Item::Mod(i) => Some(&mut i.attrs),
        syn::Item::Const(i) => Some(&mut i.attrs),
        syn::Item::Fn(i) => Some(&mut i.attrs),
        syn::Item::Static(i) => Some(&mut i.attrs),
        syn::Item::Type(i) => Some(&mut i.attrs),
        syn::Item::Trait(i) => Some(&mut i.attrs),
        syn::Item::Union(i) => Some(&mut i.attrs),
        _ => None,
    }
}

/// Collects the identity of every oneof enum in the file (recursing into modules) whose variants
/// are all non-borrowing, i.e. that will be emitted without a lifetime parameter.
///
/// A oneof is always nested in its containing message and is referenced only by that message, and
/// prost-build emits both in the same module file (the oneof enum in a submodule named after the
/// message), so a single-file pass sees the definition and every reference. Enums are keyed by
/// `parent_module::Enum` rather than by bare name, so two same-named oneofs in one file (e.g. two
/// messages that each declare `oneof kind`) are never conflated.
fn collect_scalar_only_oneofs(items: &[syn::Item]) -> collections::HashSet<String> {
    let mut names = collections::HashSet::new();
    collect_scalar_only_oneofs_into(items, None, &mut names);
    names
}

fn collect_scalar_only_oneofs_into(
    items: &[syn::Item],
    parent_module: Option<&syn::Ident>,
    names: &mut collections::HashSet<String>,
) {
    for item in items {
        match *item {
            syn::Item::Enum(ref enum_item)
                if has_oneof_derive(enum_item) && !oneof_borrows(enum_item) =>
            {
                names.insert(qualified_oneof_key(parent_module, &enum_item.ident));
            }
            syn::Item::Mod(syn::ItemMod {
                ident: ref module_ident,
                content: Some((_, ref items)),
                ..
            }) => collect_scalar_only_oneofs_into(items, Some(module_ident), names),
            _ => {}
        }
    }
}

/// Builds the lookup key for a oneof enum from the module it lives in and its name.
fn qualified_oneof_key(parent_module: Option<&syn::Ident>, ident: &syn::Ident) -> String {
    match parent_module {
        Some(module) => format!("{module}::{ident}"),
        None => ident.to_string(),
    }
}

/// Returns `true` if any variant of the oneof borrows from the decode buffer — i.e. is a `string`,
/// `bytes`, `message` or `group` field — and therefore needs a lifetime parameter.
fn oneof_borrows(enum_item: &syn::ItemEnum) -> bool {
    enum_item.variants.iter().any(|variant| {
        variant.attrs.iter().any(|attr| {
            attr.meta.path().is_ident("prost")
                && attr
                    .parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )
                    .map(|nested| {
                        nested.iter().any(|meta| {
                            let p = meta.path();
                            p.is_ident("string")
                                || p.is_ident("bytes")
                                || p.is_ident("message")
                                || p.is_ident("group")
                        })
                    })
                    .unwrap_or(false)
        })
    })
}

fn transform_item(item: &mut syn::Item, scalar_only_oneofs: &collections::HashSet<String>) {
    match *item {
        syn::Item::Struct(ref mut struct_item) if has_message_derive(struct_item) => {
            struct_item
                .generics
                .params
                .push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
                    syn::Lifetime::new("'a", proc_macro2::Span::call_site()),
                )));

            for field in &mut struct_item.fields {
                transform_field(field, scalar_only_oneofs);
            }
            match struct_item.fields {
                syn::Fields::Named(syn::FieldsNamed { ref mut named, .. }) => {
                    let dummy_struct: syn::ItemStruct = syn::parse2(quote::quote! {
                        struct Dummy<'a> {
                            #[femtopb(unknown_fields)]
                            pub unknown_fields: femtopb::UnknownFields<'a>
                        }
                    })
                    .unwrap();
                    named.push(dummy_struct.fields.into_iter().next().unwrap());
                }
                _ => unreachable!(),
            }
        }
        syn::Item::Enum(ref mut enum_item) if has_enum_derive(enum_item) => {
            if let Some(variant) = enum_item.variants.iter_mut().next() {
                enum_item.attrs.push(syn::parse_quote!(#[derive(Default)]));
                variant.attrs.push(syn::parse_quote!(#[default]))
            }
        }
        syn::Item::Enum(ref mut enum_item) if has_oneof_derive(enum_item) => {
            enum_item.attrs.push(syn::parse_quote! {
                #[non_exhaustive]
            });

            // A oneof only needs a lifetime — and the `PhantomData` variant that consumes it — if
            // some variant borrows from the buffer. A purely scalar oneof gets neither, since an
            // unused lifetime parameter would fail to compile. Determined directly from the (still
            // prost-annotated) variants, consistent with how the reference set was collected.
            let borrows = oneof_borrows(enum_item);
            if borrows {
                enum_item.generics.params.push(syn::GenericParam::Lifetime(
                    syn::LifetimeParam::new(syn::Lifetime::new(
                        "'a",
                        proc_macro2::Span::call_site(),
                    )),
                ));
            }

            for variant in &mut enum_item.variants {
                transform_variant(variant, scalar_only_oneofs);
            }

            if borrows {
                enum_item.variants.push(syn::parse_quote! {
                    #[femtopb(phantom)]
                    _Phantom(::core::marker::PhantomData<&'a ()>)
                });
            }
        }
        syn::Item::Mod(ref mut item_mod) => {
            if let Some(ref mut content) = item_mod.content {
                for mod_item in &mut content.1 {
                    transform_item(mod_item, scalar_only_oneofs);
                }
            }
        }
        _ => (),
    }
}

fn transform_prost_attr(attr: &mut syn::Attribute, metadata: &mut FieldMetadata) {
    if attr.meta.path().is_ident("prost") {
        let nested = attr
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .unwrap();
        // The `femtopb` arguments, assembled in the order prost emitted them. A repeated field's
        // label (`repeated` vs `packed`) cannot be decided as soon as it is seen — prost writes
        // `repeated` first and may follow it with `packed="false"` — so its position is reserved in
        // `label_slot` and filled in once the whole attribute has been read. Deciding it in one
        // place, from the whole attribute, keeps the emitted word and the `is_repeated`/`is_packed`
        // flags (which choose the Rust field type) from ever disagreeing, whatever order the
        // arguments arrive in.
        let mut parts: Vec<String> = Vec::new();
        let mut label_slot: Option<usize> = None;
        // `Some` once prost has stated the packing explicitly via `packed="true"`/`packed="false"`;
        // otherwise the label falls back to whether the element type can be packed at all.
        let mut explicit_packed: Option<bool> = None;

        for meta in nested.iter() {
            let path = meta.path();
            // Most of these are the same as in prost:
            if path.is_ident("float")
                || path.is_ident("double")
                || path.is_ident("int32")
                || path.is_ident("int64")
                || path.is_ident("uint32")
                || path.is_ident("uint64")
                || path.is_ident("sint32")
                || path.is_ident("sint64")
                || path.is_ident("fixed32")
                || path.is_ident("fixed64")
                || path.is_ident("sfixed32")
                || path.is_ident("sfixed64")
                || path.is_ident("bool")
                || path.is_ident("string")
                || path.is_ident("bytes")
            {
                let name = path.segments[0].ident.to_string();
                parts.push(name.clone());
                metadata.is_scalar = Some(name);
            } else if path.is_ident("optional")
                || path.is_ident("required")
                || path.is_ident("map")
                || path.is_ident("hash_map")
                || path.is_ident("btree_map")
            {
                parts.push(path.segments[0].ident.to_string());
            } else if path.is_ident("boxed") {
                parts.push("deferred".to_string());
            } else if path.is_ident("message") || path.is_ident("group") {
                metadata.is_message = true;
                parts.push(path.segments[0].ident.to_string());
            } else if path.is_ident("repeated") {
                label_slot.get_or_insert_with(|| {
                    parts.push(String::new());
                    parts.len() - 1
                });
            } else if path.is_ident("packed") {
                let name_value = meta.require_name_value().unwrap();
                match &name_value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(str),
                        ..
                    }) => {
                        explicit_packed = Some(match str.value().as_str() {
                            "true" => true,
                            "false" => false,
                            other => panic!("unexpected prost `packed` value: {other:?}"),
                        });
                        // prost always writes `repeated` before `packed=...`, so the slot normally
                        // already exists; reserve one anyway rather than dropping the label.
                        label_slot.get_or_insert_with(|| {
                            parts.push(String::new());
                            parts.len() - 1
                        });
                    }
                    _ => unreachable!(),
                }
            } else if path.is_ident("enumeration") {
                let name_value = meta.require_name_value().unwrap();
                match &name_value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(str),
                        ..
                    }) => {
                        metadata.is_enum = Some(syn::parse_str(str.value().as_str()).unwrap());
                        parts.push(path.segments[0].ident.to_string());
                    }
                    _ => unreachable!(),
                }
            } else if path.is_ident("oneof") {
                let name_value = meta.require_name_value().unwrap();
                match &name_value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(str),
                        ..
                    }) => {
                        metadata.is_oneof = Some(syn::parse_str(str.value().as_str()).unwrap());
                        parts.push(path.segments[0].ident.to_string());
                    }
                    _ => unreachable!(),
                }
            } else if path.is_ident("tag") {
                let name_value = meta.require_name_value().unwrap();
                match &name_value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(str),
                        ..
                    }) => {
                        parts.push(format!("tag = {}", str.value()));
                    }
                    _ => unreachable!(),
                }
            } else if path.is_ident("tags") {
                let name_value = meta.require_name_value().unwrap();
                match &name_value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(str),
                        ..
                    }) => {
                        parts.push(format!("tags = [{}]", str.value()));
                    }
                    _ => unreachable!(),
                }
            } else if path.is_ident("default") {
                let name_value = meta.require_name_value().unwrap();
                match &name_value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(str),
                        ..
                    }) => {
                        let value = if metadata.is_scalar.as_deref() == Some("string") {
                            format!("{:?}", str.value())
                        } else if let Some(e) = metadata.is_enum.as_ref() {
                            let ident: syn::Ident = syn::parse_str(str.value().as_str()).unwrap();
                            quote::quote!(#e::#ident).to_string()
                        } else {
                            str.value()
                        };
                        parts.push(format!("default = {value}"));
                    }
                    _ => unreachable!(),
                }
            } else {
                panic!("unhandled prost attr: {:?}", path.get_ident().unwrap());
            }
        }
        if let Some(slot) = label_slot {
            // Without an explicit `packed=...`, pack whatever the wire format allows to be packed:
            // that is the proto3 default, and it is what prost signals by staying silent.
            let packed = explicit_packed.unwrap_or_else(|| match metadata.is_scalar.as_deref() {
                Some(scalar) => can_pack_scalar(scalar),
                None => metadata.is_enum.is_some(),
            });
            metadata.is_packed = packed;
            metadata.is_repeated = !packed;
            parts[slot] = if packed { "packed" } else { "repeated" }.to_string();
        }
        let new_attr = format!("femtopb({})", parts.join(", "));
        let new_meta: syn::Meta = syn::parse_str(&new_attr).unwrap();
        attr.meta = new_meta;
    }
}

fn transform_field(field: &mut syn::Field, scalar_only_oneofs: &collections::HashSet<String>) {
    let mut metadata = FieldMetadata::default();
    for attr in &mut field.attrs {
        transform_prost_attr(attr, &mut metadata);
    }
    transform_field_type(&mut field.ty, &metadata, scalar_only_oneofs);
}

fn transform_variant(
    variant: &mut syn::Variant,
    scalar_only_oneofs: &collections::HashSet<String>,
) {
    let mut metadata = FieldMetadata::default();
    for attr in &mut variant.attrs {
        transform_prost_attr(attr, &mut metadata);
    }
    transform_field_type(
        &mut variant.fields.iter_mut().next().unwrap().ty,
        &metadata,
        scalar_only_oneofs,
    );
}

fn transform_field_type(
    ty: &mut syn::Type,
    metadata: &FieldMetadata,
    scalar_only_oneofs: &collections::HashSet<String>,
) {
    if let syn::Type::Path(syn::TypePath { ref mut path, .. }) = ty {
        // Check for option/vec/box before is_enum/is_message to handle the optional/repeated
        // messages/enums
        if has_same_path_idents(path, "::core::option::Option") {
            let generic_segment = path.segments.last_mut().unwrap();
            transform_field_type(
                get_single_generic_arg(generic_segment),
                metadata,
                scalar_only_oneofs,
            );
        } else if has_same_path_idents(path, "::femtopb::alloc::boxed::Box")
            || has_same_path_idents(path, "::prost::alloc::boxed::Box")
        {
            let generic_segment = path.segments.last_mut().unwrap();
            let inner_ty = get_single_generic_arg(generic_segment);
            transform_field_type(inner_ty, metadata, scalar_only_oneofs);
            *ty = syn::parse2(quote::quote!(::femtopb::deferred::Deferred<'a, #inner_ty>)).unwrap();
        } else if has_same_path_idents(path, "::femtopb::alloc::vec::Vec")
            || has_same_path_idents(path, "::prost::alloc::vec::Vec")
        {
            let generic_segment = path.segments.last_mut().unwrap();
            let inner_ty = get_single_generic_arg(generic_segment);

            let base_type = if metadata.is_repeated {
                quote::quote!(::femtopb::repeated::Repeated)
            } else if metadata.is_packed {
                quote::quote!(::femtopb::packed::Packed)
            } else {
                panic!("Found vec field but field is not repeated or packed!")
            };
            let item_encoding = match metadata.is_scalar.as_deref() {
                Some("float") => {
                    quote::quote!(::femtopb::item_encoding::Float)
                }
                Some("double") => {
                    quote::quote!(::femtopb::item_encoding::Double)
                }
                Some("int32") => {
                    quote::quote!(::femtopb::item_encoding::Int32)
                }
                Some("int64") => {
                    quote::quote!(::femtopb::item_encoding::Int64)
                }
                Some("uint32") => {
                    quote::quote!(::femtopb::item_encoding::UInt32)
                }
                Some("uint64") => {
                    quote::quote!(::femtopb::item_encoding::UInt64)
                }
                Some("sint32") => {
                    quote::quote!(::femtopb::item_encoding::SInt32)
                }
                Some("sint64") => {
                    quote::quote!(::femtopb::item_encoding::SInt64)
                }
                Some("fixed32") => {
                    quote::quote!(::femtopb::item_encoding::Fixed32)
                }
                Some("fixed64") => {
                    quote::quote!(::femtopb::item_encoding::Fixed64)
                }
                Some("sfixed32") => {
                    quote::quote!(::femtopb::item_encoding::SFixed32)
                }
                Some("sfixed64") => {
                    quote::quote!(::femtopb::item_encoding::SFixed64)
                }
                Some("bool") => {
                    quote::quote!(::femtopb::item_encoding::Bool)
                }
                Some("string") => {
                    quote::quote!(::femtopb::item_encoding::String)
                }
                Some("bytes") => {
                    quote::quote!(::femtopb::item_encoding::Bytes)
                }
                None => {
                    if metadata.is_message {
                        quote::quote!(::femtopb::item_encoding::Message<'a, #inner_ty<'a>>)
                    } else if let Some(ref e) = metadata.is_enum {
                        quote::quote!(::femtopb::item_encoding::Enum<#e>)
                    } else {
                        panic!("unable to determine item encoding!")
                    }
                }
                Some(v) => panic!("unable to determine item encoding for {:?}", v),
            };
            transform_field_type(inner_ty, metadata, scalar_only_oneofs);
            *ty = syn::parse2(quote::quote!(#base_type<'a, #inner_ty, #item_encoding>)).unwrap();
        } else if metadata.is_message || metadata.is_oneof.is_some() {
            // A scalar-only oneof is emitted without a lifetime parameter, so references to it
            // must not add one; messages (and borrowing oneofs) always carry `<'a>`. The oneof
            // is referenced as `parent_module::Enum`, matching the key used during collection.
            let is_scalar_only_oneof = metadata.is_oneof.is_some() && {
                let n = path.segments.len();
                let parent = (n >= 2).then(|| path.segments[n - 2].ident.clone());
                let key = qualified_oneof_key(parent.as_ref(), &path.segments[n - 1].ident);
                scalar_only_oneofs.contains(&key)
            };
            let generic_segment = path.segments.last_mut().unwrap();
            let ident = &generic_segment.ident;
            if !is_scalar_only_oneof {
                *generic_segment = syn::parse2(quote::quote!(#ident<'a>)).unwrap();
            }
        } else if let Some(enum_ty) = &metadata.is_enum {
            *ty = syn::parse2(quote::quote!(::femtopb::enumeration::EnumValue<#enum_ty>)).unwrap();
        } else if has_same_path_idents(path, "::femtopb::alloc::string::String")
            || has_same_path_idents(path, "::prost::alloc::string::String")
        {
            *ty = syn::parse2(quote::quote!(&'a str)).unwrap();
        } else if has_same_path_idents(path, "::prost::bytes::Bytes")
            || has_same_path_idents(path, "::femtopb::bytes::Bytes")
        {
            *ty = syn::parse2(quote::quote!(&'a [u8])).unwrap();
        }
    }
}

/// Returns `true` if the repeated field type can be packed.
fn can_pack_scalar(field: &str) -> bool {
    matches!(
        field,
        "float"
            | "double"
            | "int32"
            | "int64"
            | "uint32"
            | "uint64"
            | "sint32"
            | "sint64"
            | "fixed32"
            | "fixed64"
            | "sfixed32"
            | "sfixed64"
            | "bool"
    )
}

fn has_message_derive(struct_item: &syn::ItemStruct) -> bool {
    struct_item
        .attrs
        .iter()
        .find(|a| is_derive_attr(a, "::femtopb::Message"))
        .is_some()
}

fn has_enum_derive(enum_item: &syn::ItemEnum) -> bool {
    enum_item
        .attrs
        .iter()
        .find(|a| is_derive_attr(a, "::femtopb::Enumeration"))
        .is_some()
}

fn has_oneof_derive(enum_item: &syn::ItemEnum) -> bool {
    enum_item
        .attrs
        .iter()
        .find(|a| is_derive_attr(a, "::femtopb::Oneof"))
        .is_some()
}

fn is_derive_attr(attr: &syn::Attribute, derive: &str) -> bool {
    if attr.meta.path().is_ident("derive") {
        let mut found = false;
        attr.parse_nested_meta(|meta| {
            found = found || has_same_path_idents(&meta.path, derive);
            Ok(())
        })
        .unwrap();
        found
    } else {
        false
    }
}

fn has_same_path_idents(path: &syn::Path, other: &str) -> bool {
    // This is of course really inefficient, but the speed of this code isn't critical... we can
    // fix it if it ever becomes a problem
    let parsed_other: syn::Path = syn::parse_str(other).unwrap();
    path.leading_colon.is_some() == parsed_other.leading_colon.is_some()
        && path_segments(path) == path_segments(&parsed_other)
}

fn get_single_generic_arg(segment: &mut syn::PathSegment) -> &mut syn::Type {
    match segment.arguments {
        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            ref mut args,
            ..
        }) => match args.first_mut().unwrap() {
            syn::GenericArgument::Type(ty) => ty,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn path_segments(p: &syn::Path) -> Vec<String> {
    p.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Strips the per-item lint-suppression attributes that `transform` prepends to every generated
    /// item, so the body-comparison tests below can assert on the transformed code itself.
    fn strip_lint_allows(generated: &str) -> String {
        assert!(
            generated.contains("#[allow(clippy::all, deprecated)]"),
            "generated code should carry the per-item lint-suppression attribute"
        );
        generated
            .lines()
            .filter(|line| line.trim() != "#[allow(clippy::all, deprecated)]")
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }

    #[test]
    fn transform_message_one_scalar() {
        let original = r#"
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct NestedMessage {
    /// The field name "b" fails to compile in proto1 because it conflicts with
    /// a local variable named "b" in one of the generated methods.  Doh.
    /// This file needs to compile in proto1 to test backwards-compatibility.
    #[prost(int32, optional, tag = "1")]
    pub bb: ::core::option::Option<i32>,
}
"#;
        let expected = r#"
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct NestedMessage<'a> {
    /// The field name "b" fails to compile in proto1 because it conflicts with
    /// a local variable named "b" in one of the generated methods.  Doh.
    /// This file needs to compile in proto1 to test backwards-compatibility.
    #[femtopb(int32, optional, tag = 1)]
    pub bb: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
"#;
        let actual = transform(original);
        assert_eq!(strip_lint_allows(&actual), expected.trim());
    }

    #[test]
    fn scalar_only_oneof_omits_lifetime_and_phantom() {
        // A oneof whose variants are all scalar must be emitted without a lifetime and without the
        // `PhantomData` variant (an unused lifetime is a hard error), and references to it must not
        // add a lifetime argument. A oneof that borrows keeps all three.
        let original = r#"
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Msg {
    #[prost(oneof="msg::Scalar", tags="1")]
    pub scalar: ::core::option::Option<msg::Scalar>,
    #[prost(oneof="msg::Borrowing", tags="2")]
    pub borrowing: ::core::option::Option<msg::Borrowing>,
}
pub mod msg {
    #[derive(Clone, Copy, PartialEq, ::femtopb::Oneof)]
    pub enum Scalar {
        #[prost(int32, tag="1")]
        Number(i32),
    }
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    pub enum Borrowing {
        #[prost(string, tag="2")]
        Text(::prost::alloc::string::String),
    }
}
"#;
        let actual = transform(original);

        // Scalar-only oneof: no lifetime, no phantom variant, and a lifetime-free reference.
        assert!(actual.contains("pub enum Scalar {"), "{actual}");
        assert!(!actual.contains("Scalar<'a>"), "{actual}");
        assert!(
            actual.contains("pub scalar: ::core::option::Option<msg::Scalar>"),
            "{actual}"
        );

        // Borrowing oneof: keeps its lifetime, its phantom variant, and its qualified reference.
        assert!(actual.contains("pub enum Borrowing<'a>"), "{actual}");
        assert!(actual.contains("_Phantom"), "{actual}");
        assert!(actual.contains("Option<msg::Borrowing<'a>>"), "{actual}");
    }

    #[test]
    fn transform_message_singular_scalars() {
        let original = r#"
/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes {
    /// Singular
    #[prost(int32, optional, tag="1")]
    pub optional_int32: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="2")]
    pub optional_int64: ::core::option::Option<i64>,
    #[prost(uint32, optional, tag="3")]
    pub optional_uint32: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="4")]
    pub optional_uint64: ::core::option::Option<u64>,
    #[prost(sint32, optional, tag="5")]
    pub optional_sint32: ::core::option::Option<i32>,
    #[prost(sint64, optional, tag="6")]
    pub optional_sint64: ::core::option::Option<i64>,
    #[prost(fixed32, optional, tag="7")]
    pub optional_fixed32: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag="8")]
    pub optional_fixed64: ::core::option::Option<u64>,
    #[prost(sfixed32, optional, tag="9")]
    pub optional_sfixed32: ::core::option::Option<i32>,
    #[prost(sfixed64, optional, tag="10")]
    pub optional_sfixed64: ::core::option::Option<i64>,
    #[prost(float, optional, tag="11")]
    pub optional_float: ::core::option::Option<f32>,
    #[prost(double, optional, tag="12")]
    pub optional_double: ::core::option::Option<f64>,
    #[prost(bool, optional, tag="13")]
    pub optional_bool: ::core::option::Option<bool>,
    #[prost(string, optional, tag="14")]
    pub optional_string: ::core::option::Option<::femtopb::alloc::string::String>,
    #[prost(bytes="bytes", optional, tag="15")]
    pub optional_bytes: ::core::option::Option<::prost::bytes::Bytes>,
}
"#;
        let expected = r#"
/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes<'a> {
    /// Singular
    #[femtopb(int32, optional, tag = 1)]
    pub optional_int32: ::core::option::Option<i32>,
    #[femtopb(int64, optional, tag = 2)]
    pub optional_int64: ::core::option::Option<i64>,
    #[femtopb(uint32, optional, tag = 3)]
    pub optional_uint32: ::core::option::Option<u32>,
    #[femtopb(uint64, optional, tag = 4)]
    pub optional_uint64: ::core::option::Option<u64>,
    #[femtopb(sint32, optional, tag = 5)]
    pub optional_sint32: ::core::option::Option<i32>,
    #[femtopb(sint64, optional, tag = 6)]
    pub optional_sint64: ::core::option::Option<i64>,
    #[femtopb(fixed32, optional, tag = 7)]
    pub optional_fixed32: ::core::option::Option<u32>,
    #[femtopb(fixed64, optional, tag = 8)]
    pub optional_fixed64: ::core::option::Option<u64>,
    #[femtopb(sfixed32, optional, tag = 9)]
    pub optional_sfixed32: ::core::option::Option<i32>,
    #[femtopb(sfixed64, optional, tag = 10)]
    pub optional_sfixed64: ::core::option::Option<i64>,
    #[femtopb(float, optional, tag = 11)]
    pub optional_float: ::core::option::Option<f32>,
    #[femtopb(double, optional, tag = 12)]
    pub optional_double: ::core::option::Option<f64>,
    #[femtopb(bool, optional, tag = 13)]
    pub optional_bool: ::core::option::Option<bool>,
    #[femtopb(string, optional, tag = 14)]
    pub optional_string: ::core::option::Option<&'a str>,
    #[femtopb(bytes, optional, tag = 15)]
    pub optional_bytes: ::core::option::Option<&'a [u8]>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
"#;
        let actual = transform(original);
        assert_eq!(strip_lint_allows(&actual), expected.trim());
    }

    #[test]
    fn transform_message_singular_compounds() {
        let original = r#"
/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes {
    #[prost(group, optional, tag="16")]
    pub optionalgroup: ::core::option::Option<test_all_types::OptionalGroup>,
    #[prost(message, optional, tag="18")]
    pub optional_nested_message: ::core::option::Option<test_all_types::NestedMessage>,
    #[prost(message, optional, tag="19")]
    pub optional_foreign_message: ::core::option::Option<ForeignMessage>,
    #[prost(message, optional, tag="20")]
    pub optional_import_message: ::core::option::Option<super::protobuf_unittest_import::ImportMessage>,
    #[prost(enumeration="test_all_types::NestedEnum", optional, tag="21")]
    pub optional_nested_enum: ::core::option::Option<i32>,
    #[prost(enumeration="ForeignEnum", optional, tag="22")]
    pub optional_foreign_enum: ::core::option::Option<i32>,
    #[prost(enumeration="super::protobuf_unittest_import::ImportEnum", optional, tag="23")]
    pub optional_import_enum: ::core::option::Option<i32>,
    #[prost(string, optional, tag="24")]
    pub optional_string_piece: ::core::option::Option<::femtopb::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub optional_cord: ::core::option::Option<::femtopb::alloc::string::String>,
    /// Defined in unittest_import_public.proto
    #[prost(message, optional, tag="26")]
    pub optional_public_import_message: ::core::option::Option<super::protobuf_unittest_import::PublicImportMessage>,
    #[prost(message, optional, tag="27")]
    pub optional_lazy_message: ::core::option::Option<test_all_types::NestedMessage>,
    #[prost(message, optional, tag="28")]
    pub optional_unverified_lazy_message: ::core::option::Option<test_all_types::NestedMessage>,
}
"#;
        let expected = r#"
/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes<'a> {
    #[femtopb(group, optional, tag = 16)]
    pub optionalgroup: ::core::option::Option<test_all_types::OptionalGroup<'a>>,
    #[femtopb(message, optional, tag = 18)]
    pub optional_nested_message: ::core::option::Option<
        test_all_types::NestedMessage<'a>,
    >,
    #[femtopb(message, optional, tag = 19)]
    pub optional_foreign_message: ::core::option::Option<ForeignMessage<'a>>,
    #[femtopb(message, optional, tag = 20)]
    pub optional_import_message: ::core::option::Option<
        super::protobuf_unittest_import::ImportMessage<'a>,
    >,
    #[femtopb(enumeration, optional, tag = 21)]
    pub optional_nested_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<test_all_types::NestedEnum>,
    >,
    #[femtopb(enumeration, optional, tag = 22)]
    pub optional_foreign_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
    >,
    #[femtopb(enumeration, optional, tag = 23)]
    pub optional_import_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<super::protobuf_unittest_import::ImportEnum>,
    >,
    #[femtopb(string, optional, tag = 24)]
    pub optional_string_piece: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 25)]
    pub optional_cord: ::core::option::Option<&'a str>,
    /// Defined in unittest_import_public.proto
    #[femtopb(message, optional, tag = 26)]
    pub optional_public_import_message: ::core::option::Option<
        super::protobuf_unittest_import::PublicImportMessage<'a>,
    >,
    #[femtopb(message, optional, tag = 27)]
    pub optional_lazy_message: ::core::option::Option<test_all_types::NestedMessage<'a>>,
    #[femtopb(message, optional, tag = 28)]
    pub optional_unverified_lazy_message: ::core::option::Option<
        test_all_types::NestedMessage<'a>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
"#;
        let actual = transform(original);
        assert_eq!(strip_lint_allows(&actual), expected.trim());
    }

    #[test]
    fn transform_message_repeated_scalars() {
        let original = r#"
/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes {
    /// Repeated
    #[prost(int32, repeated, packed="false", tag="31")]
    pub repeated_int32: ::femtopb::alloc::vec::Vec<i32>,
    #[prost(int64, repeated, packed="false", tag="32")]
    pub repeated_int64: ::femtopb::alloc::vec::Vec<i64>,
    #[prost(uint32, repeated, packed="false", tag="33")]
    pub repeated_uint32: ::femtopb::alloc::vec::Vec<u32>,
    #[prost(uint64, repeated, packed="false", tag="34")]
    pub repeated_uint64: ::femtopb::alloc::vec::Vec<u64>,
    #[prost(sint32, repeated, packed="false", tag="35")]
    pub repeated_sint32: ::femtopb::alloc::vec::Vec<i32>,
    #[prost(sint64, repeated, packed="false", tag="36")]
    pub repeated_sint64: ::femtopb::alloc::vec::Vec<i64>,
    #[prost(fixed32, repeated, packed="false", tag="37")]
    pub repeated_fixed32: ::femtopb::alloc::vec::Vec<u32>,
    #[prost(fixed64, repeated, packed="false", tag="38")]
    pub repeated_fixed64: ::femtopb::alloc::vec::Vec<u64>,
    #[prost(sfixed32, repeated, packed="false", tag="39")]
    pub repeated_sfixed32: ::femtopb::alloc::vec::Vec<i32>,
    #[prost(sfixed64, repeated, packed="false", tag="40")]
    pub repeated_sfixed64: ::femtopb::alloc::vec::Vec<i64>,
    #[prost(float, repeated, packed="false", tag="41")]
    pub repeated_float: ::femtopb::alloc::vec::Vec<f32>,
    #[prost(double, repeated, packed="false", tag="42")]
    pub repeated_double: ::femtopb::alloc::vec::Vec<f64>,
    #[prost(bool, repeated, packed="false", tag="43")]
    pub repeated_bool: ::femtopb::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag="44")]
    pub repeated_string: ::femtopb::alloc::vec::Vec<::femtopb::alloc::string::String>,
    #[prost(bytes="bytes", repeated, tag="45")]
    pub repeated_bytes: ::femtopb::alloc::vec::Vec<::prost::bytes::Bytes>,
}
"#;
        let expected = r#"
/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes<'a> {
    /// Repeated
    #[femtopb(int32, repeated, tag = 31)]
    pub repeated_int32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int64, repeated, tag = 32)]
    pub repeated_int64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    #[femtopb(uint32, repeated, tag = 33)]
    pub repeated_uint32: ::femtopb::repeated::Repeated<
        'a,
        u32,
        ::femtopb::item_encoding::UInt32,
    >,
    #[femtopb(uint64, repeated, tag = 34)]
    pub repeated_uint64: ::femtopb::repeated::Repeated<
        'a,
        u64,
        ::femtopb::item_encoding::UInt64,
    >,
    #[femtopb(sint32, repeated, tag = 35)]
    pub repeated_sint32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::SInt32,
    >,
    #[femtopb(sint64, repeated, tag = 36)]
    pub repeated_sint64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::SInt64,
    >,
    #[femtopb(fixed32, repeated, tag = 37)]
    pub repeated_fixed32: ::femtopb::repeated::Repeated<
        'a,
        u32,
        ::femtopb::item_encoding::Fixed32,
    >,
    #[femtopb(fixed64, repeated, tag = 38)]
    pub repeated_fixed64: ::femtopb::repeated::Repeated<
        'a,
        u64,
        ::femtopb::item_encoding::Fixed64,
    >,
    #[femtopb(sfixed32, repeated, tag = 39)]
    pub repeated_sfixed32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::SFixed32,
    >,
    #[femtopb(sfixed64, repeated, tag = 40)]
    pub repeated_sfixed64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::SFixed64,
    >,
    #[femtopb(float, repeated, tag = 41)]
    pub repeated_float: ::femtopb::repeated::Repeated<
        'a,
        f32,
        ::femtopb::item_encoding::Float,
    >,
    #[femtopb(double, repeated, tag = 42)]
    pub repeated_double: ::femtopb::repeated::Repeated<
        'a,
        f64,
        ::femtopb::item_encoding::Double,
    >,
    #[femtopb(bool, repeated, tag = 43)]
    pub repeated_bool: ::femtopb::repeated::Repeated<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    #[femtopb(string, repeated, tag = 44)]
    pub repeated_string: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(bytes, repeated, tag = 45)]
    pub repeated_bytes: ::femtopb::repeated::Repeated<
        'a,
        &'a [u8],
        ::femtopb::item_encoding::Bytes,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
"#;
        let actual = transform(original);
        assert_eq!(strip_lint_allows(&actual), expected.trim());
    }

    #[test]
    fn transform_message_singulars_with_defaults() {
        let original = r#"
/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes {
    #[prost(int32, optional, tag="61", default="41")]
    pub default_int32: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="62", default="42")]
    pub default_int64: ::core::option::Option<i64>,
    #[prost(uint32, optional, tag="63", default="43")]
    pub default_uint32: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="64", default="44")]
    pub default_uint64: ::core::option::Option<u64>,
    #[prost(sint32, optional, tag="65", default="-45")]
    pub default_sint32: ::core::option::Option<i32>,
    #[prost(sint64, optional, tag="66", default="46")]
    pub default_sint64: ::core::option::Option<i64>,
    #[prost(fixed32, optional, tag="67", default="47")]
    pub default_fixed32: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag="68", default="48")]
    pub default_fixed64: ::core::option::Option<u64>,
    #[prost(sfixed32, optional, tag="69", default="49")]
    pub default_sfixed32: ::core::option::Option<i32>,
    #[prost(sfixed64, optional, tag="70", default="-50")]
    pub default_sfixed64: ::core::option::Option<i64>,
    #[prost(float, optional, tag="71", default="51.5")]
    pub default_float: ::core::option::Option<f32>,
    #[prost(double, optional, tag="72", default="52000")]
    pub default_double: ::core::option::Option<f64>,
    #[prost(bool, optional, tag="73", default="true")]
    pub default_bool: ::core::option::Option<bool>,
    #[prost(string, optional, tag="74", default="hello")]
    pub default_string: ::core::option::Option<::femtopb::alloc::string::String>,
    #[prost(bytes="bytes", optional, tag="75", default="b\"world\"")]
    pub default_bytes: ::core::option::Option<::prost::bytes::Bytes>,
    #[prost(enumeration="test_all_types::NestedEnum", optional, tag="81", default="Bar")]
    pub default_nested_enum: ::core::option::Option<i32>,
    #[prost(enumeration="ForeignEnum", optional, tag="82", default="ForeignBar")]
    pub default_foreign_enum: ::core::option::Option<i32>,
    #[prost(enumeration="super::protobuf_unittest_import::ImportEnum", optional, tag="83", default="ImportBar")]
    pub default_import_enum: ::core::option::Option<i32>,
    #[prost(string, optional, tag="84", default="abc")]
    pub default_string_piece: ::core::option::Option<::femtopb::alloc::string::String>,
    #[prost(string, optional, tag="85", default="123")]
    pub default_cord: ::core::option::Option<::femtopb::alloc::string::String>,
}
"#;
        let expected = r#"
/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes<'a> {
    #[femtopb(int32, optional, tag = 61, default = 41)]
    pub default_int32: ::core::option::Option<i32>,
    #[femtopb(int64, optional, tag = 62, default = 42)]
    pub default_int64: ::core::option::Option<i64>,
    #[femtopb(uint32, optional, tag = 63, default = 43)]
    pub default_uint32: ::core::option::Option<u32>,
    #[femtopb(uint64, optional, tag = 64, default = 44)]
    pub default_uint64: ::core::option::Option<u64>,
    #[femtopb(sint32, optional, tag = 65, default = -45)]
    pub default_sint32: ::core::option::Option<i32>,
    #[femtopb(sint64, optional, tag = 66, default = 46)]
    pub default_sint64: ::core::option::Option<i64>,
    #[femtopb(fixed32, optional, tag = 67, default = 47)]
    pub default_fixed32: ::core::option::Option<u32>,
    #[femtopb(fixed64, optional, tag = 68, default = 48)]
    pub default_fixed64: ::core::option::Option<u64>,
    #[femtopb(sfixed32, optional, tag = 69, default = 49)]
    pub default_sfixed32: ::core::option::Option<i32>,
    #[femtopb(sfixed64, optional, tag = 70, default = -50)]
    pub default_sfixed64: ::core::option::Option<i64>,
    #[femtopb(float, optional, tag = 71, default = 51.5)]
    pub default_float: ::core::option::Option<f32>,
    #[femtopb(double, optional, tag = 72, default = 52000)]
    pub default_double: ::core::option::Option<f64>,
    #[femtopb(bool, optional, tag = 73, default = true)]
    pub default_bool: ::core::option::Option<bool>,
    #[femtopb(string, optional, tag = 74, default = "hello")]
    pub default_string: ::core::option::Option<&'a str>,
    #[femtopb(bytes, optional, tag = 75, default = b"world")]
    pub default_bytes: ::core::option::Option<&'a [u8]>,
    #[femtopb(
        enumeration,
        optional,
        tag = 81,
        default = test_all_types::NestedEnum::Bar
    )]
    pub default_nested_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<test_all_types::NestedEnum>,
    >,
    #[femtopb(enumeration, optional, tag = 82, default = ForeignEnum::ForeignBar)]
    pub default_foreign_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
    >,
    #[femtopb(
        enumeration,
        optional,
        tag = 83,
        default = super::protobuf_unittest_import::ImportEnum::ImportBar
    )]
    pub default_import_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<super::protobuf_unittest_import::ImportEnum>,
    >,
    #[femtopb(string, optional, tag = 84, default = "abc")]
    pub default_string_piece: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 85, default = "123")]
    pub default_cord: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
"#;
        let actual = transform(original);
        assert_eq!(strip_lint_allows(&actual), expected.trim());
    }

    #[test]
    fn transform_enum() {
        let original = r#"
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::femtopb::Enumeration)]
#[repr(i32)]
pub enum NestedEnum {
    Foo = 1,
    Bar = 2,
    Baz = 3,
    /// Intentionally negative.
    Neg = -1,
}
"#;
        let expected = r#"
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum NestedEnum {
    #[default]
    Foo = 1,
    Bar = 2,
    Baz = 3,
    /// Intentionally negative.
    Neg = -1,
}
"#;
        let actual = transform(original);
        assert_eq!(strip_lint_allows(&actual), expected.trim());
    }

    #[test]
    #[ignore]
    fn transform_message_all_types() {
        // This test only really exists to house the full TestAllTypes to aid in copy-pasting
        let original = r#"
/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes {
    /// Singular
    #[prost(int32, optional, tag="1")]
    pub optional_int32: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="2")]
    pub optional_int64: ::core::option::Option<i64>,
    #[prost(uint32, optional, tag="3")]
    pub optional_uint32: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="4")]
    pub optional_uint64: ::core::option::Option<u64>,
    #[prost(sint32, optional, tag="5")]
    pub optional_sint32: ::core::option::Option<i32>,
    #[prost(sint64, optional, tag="6")]
    pub optional_sint64: ::core::option::Option<i64>,
    #[prost(fixed32, optional, tag="7")]
    pub optional_fixed32: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag="8")]
    pub optional_fixed64: ::core::option::Option<u64>,
    #[prost(sfixed32, optional, tag="9")]
    pub optional_sfixed32: ::core::option::Option<i32>,
    #[prost(sfixed64, optional, tag="10")]
    pub optional_sfixed64: ::core::option::Option<i64>,
    #[prost(float, optional, tag="11")]
    pub optional_float: ::core::option::Option<f32>,
    #[prost(double, optional, tag="12")]
    pub optional_double: ::core::option::Option<f64>,
    #[prost(bool, optional, tag="13")]
    pub optional_bool: ::core::option::Option<bool>,
    #[prost(string, optional, tag="14")]
    pub optional_string: ::core::option::Option<::femtopb::alloc::string::String>,
    #[prost(bytes="bytes", optional, tag="15")]
    pub optional_bytes: ::core::option::Option<::prost::bytes::Bytes>,
    #[prost(group, optional, tag="16")]
    pub optionalgroup: ::core::option::Option<test_all_types::OptionalGroup>,
    #[prost(message, optional, tag="18")]
    pub optional_nested_message: ::core::option::Option<test_all_types::NestedMessage>,
    #[prost(message, optional, tag="19")]
    pub optional_foreign_message: ::core::option::Option<ForeignMessage>,
    #[prost(message, optional, tag="20")]
    pub optional_import_message: ::core::option::Option<super::protobuf_unittest_import::ImportMessage>,
    #[prost(enumeration="test_all_types::NestedEnum", optional, tag="21")]
    pub optional_nested_enum: ::core::option::Option<i32>,
    #[prost(enumeration="ForeignEnum", optional, tag="22")]
    pub optional_foreign_enum: ::core::option::Option<i32>,
    #[prost(enumeration="super::protobuf_unittest_import::ImportEnum", optional, tag="23")]
    pub optional_import_enum: ::core::option::Option<i32>,
    #[prost(string, optional, tag="24")]
    pub optional_string_piece: ::core::option::Option<::femtopb::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub optional_cord: ::core::option::Option<::femtopb::alloc::string::String>,
    /// Defined in unittest_import_public.proto
    #[prost(message, optional, tag="26")]
    pub optional_public_import_message: ::core::option::Option<super::protobuf_unittest_import::PublicImportMessage>,
    #[prost(message, optional, tag="27")]
    pub optional_lazy_message: ::core::option::Option<test_all_types::NestedMessage>,
    #[prost(message, optional, tag="28")]
    pub optional_unverified_lazy_message: ::core::option::Option<test_all_types::NestedMessage>,
    /// Repeated
    #[prost(int32, repeated, packed="false", tag="31")]
    pub repeated_int32: ::femtopb::alloc::vec::Vec<i32>,
    #[prost(int64, repeated, packed="false", tag="32")]
    pub repeated_int64: ::femtopb::alloc::vec::Vec<i64>,
    #[prost(uint32, repeated, packed="false", tag="33")]
    pub repeated_uint32: ::femtopb::alloc::vec::Vec<u32>,
    #[prost(uint64, repeated, packed="false", tag="34")]
    pub repeated_uint64: ::femtopb::alloc::vec::Vec<u64>,
    #[prost(sint32, repeated, packed="false", tag="35")]
    pub repeated_sint32: ::femtopb::alloc::vec::Vec<i32>,
    #[prost(sint64, repeated, packed="false", tag="36")]
    pub repeated_sint64: ::femtopb::alloc::vec::Vec<i64>,
    #[prost(fixed32, repeated, packed="false", tag="37")]
    pub repeated_fixed32: ::femtopb::alloc::vec::Vec<u32>,
    #[prost(fixed64, repeated, packed="false", tag="38")]
    pub repeated_fixed64: ::femtopb::alloc::vec::Vec<u64>,
    #[prost(sfixed32, repeated, packed="false", tag="39")]
    pub repeated_sfixed32: ::femtopb::alloc::vec::Vec<i32>,
    #[prost(sfixed64, repeated, packed="false", tag="40")]
    pub repeated_sfixed64: ::femtopb::alloc::vec::Vec<i64>,
    #[prost(float, repeated, packed="false", tag="41")]
    pub repeated_float: ::femtopb::alloc::vec::Vec<f32>,
    #[prost(double, repeated, packed="false", tag="42")]
    pub repeated_double: ::femtopb::alloc::vec::Vec<f64>,
    #[prost(bool, repeated, packed="false", tag="43")]
    pub repeated_bool: ::femtopb::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag="44")]
    pub repeated_string: ::femtopb::alloc::vec::Vec<::femtopb::alloc::string::String>,
    #[prost(bytes="bytes", repeated, tag="45")]
    pub repeated_bytes: ::femtopb::alloc::vec::Vec<::prost::bytes::Bytes>,
    #[prost(group, repeated, tag="46")]
    pub repeatedgroup: ::femtopb::alloc::vec::Vec<test_all_types::RepeatedGroup>,
    #[prost(message, repeated, tag="48")]
    pub repeated_nested_message: ::femtopb::alloc::vec::Vec<test_all_types::NestedMessage>,
    #[prost(message, repeated, tag="49")]
    pub repeated_foreign_message: ::femtopb::alloc::vec::Vec<ForeignMessage>,
    #[prost(message, repeated, tag="50")]
    pub repeated_import_message: ::femtopb::alloc::vec::Vec<super::protobuf_unittest_import::ImportMessage>,
    #[prost(enumeration="test_all_types::NestedEnum", repeated, packed="false", tag="51")]
    pub repeated_nested_enum: ::femtopb::alloc::vec::Vec<i32>,
    #[prost(enumeration="ForeignEnum", repeated, packed="false", tag="52")]
    pub repeated_foreign_enum: ::femtopb::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::protobuf_unittest_import::ImportEnum", repeated, packed="false", tag="53")]
    pub repeated_import_enum: ::femtopb::alloc::vec::Vec<i32>,
    #[prost(string, repeated, tag="54")]
    pub repeated_string_piece: ::femtopb::alloc::vec::Vec<::femtopb::alloc::string::String>,
    #[prost(string, repeated, tag="55")]
    pub repeated_cord: ::femtopb::alloc::vec::Vec<::femtopb::alloc::string::String>,
    #[prost(message, repeated, tag="57")]
    pub repeated_lazy_message: ::femtopb::alloc::vec::Vec<test_all_types::NestedMessage>,
    /// Singular with defaults
    #[prost(int32, optional, tag="61", default="41")]
    pub default_int32: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="62", default="42")]
    pub default_int64: ::core::option::Option<i64>,
    #[prost(uint32, optional, tag="63", default="43")]
    pub default_uint32: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="64", default="44")]
    pub default_uint64: ::core::option::Option<u64>,
    #[prost(sint32, optional, tag="65", default="-45")]
    pub default_sint32: ::core::option::Option<i32>,
    #[prost(sint64, optional, tag="66", default="46")]
    pub default_sint64: ::core::option::Option<i64>,
    #[prost(fixed32, optional, tag="67", default="47")]
    pub default_fixed32: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag="68", default="48")]
    pub default_fixed64: ::core::option::Option<u64>,
    #[prost(sfixed32, optional, tag="69", default="49")]
    pub default_sfixed32: ::core::option::Option<i32>,
    #[prost(sfixed64, optional, tag="70", default="-50")]
    pub default_sfixed64: ::core::option::Option<i64>,
    #[prost(float, optional, tag="71", default="51.5")]
    pub default_float: ::core::option::Option<f32>,
    #[prost(double, optional, tag="72", default="52000")]
    pub default_double: ::core::option::Option<f64>,
    #[prost(bool, optional, tag="73", default="true")]
    pub default_bool: ::core::option::Option<bool>,
    #[prost(string, optional, tag="74", default="hello")]
    pub default_string: ::core::option::Option<::femtopb::alloc::string::String>,
    #[prost(bytes="bytes", optional, tag="75", default="b\"world\"")]
    pub default_bytes: ::core::option::Option<::prost::bytes::Bytes>,
    #[prost(enumeration="test_all_types::NestedEnum", optional, tag="81", default="Bar")]
    pub default_nested_enum: ::core::option::Option<i32>,
    #[prost(enumeration="ForeignEnum", optional, tag="82", default="ForeignBar")]
    pub default_foreign_enum: ::core::option::Option<i32>,
    #[prost(enumeration="super::protobuf_unittest_import::ImportEnum", optional, tag="83", default="ImportBar")]
    pub default_import_enum: ::core::option::Option<i32>,
    #[prost(string, optional, tag="84", default="abc")]
    pub default_string_piece: ::core::option::Option<::femtopb::alloc::string::String>,
    #[prost(string, optional, tag="85", default="123")]
    pub default_cord: ::core::option::Option<::femtopb::alloc::string::String>,
    /// For oneof test
    #[prost(oneof="test_all_types::OneofField", tags="111, 112, 113, 114, 115, 116, 117")]
    pub oneof_field: ::core::option::Option<test_all_types::OneofField>,
}
"#;
        let expected = r#"
"#;
        let actual = transform(original);
        assert_eq!(strip_lint_allows(&actual), expected.trim());
    }

    #[test]
    fn transform_boxed_message_becomes_deferred() {
        // prost boxes recursive/large messages (`boxed`, `Box<T>`); femtopb lowers that to a lazily
        // parsed `Deferred`. Neither the attribute rewrite (`boxed` -> `deferred`) nor the type
        // rewrite is exercised by any other test.
        let original = r#"
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Outer {
    #[prost(message, optional, boxed, tag="1")]
    pub inner: ::core::option::Option<::prost::alloc::boxed::Box<Inner>>,
}
"#;
        let actual = transform(original);
        assert!(
            actual.contains("#[femtopb(message, optional, deferred, tag = 1)]"),
            "{actual}"
        );
        assert!(
            actual.contains("::femtopb::deferred::Deferred<'a, Inner<'a>>"),
            "{actual}"
        );
    }

    #[test]
    fn transform_default_packed_scalar_becomes_packed() {
        // A repeated scalar with no explicit `packed` is packable by default, so it must lower to
        // `Packed`, not `Repeated`. Every other repeated-scalar test pins `packed="false"`, so the
        // `Packed` output branch is otherwise never taken.
        let original = r#"
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct M {
    #[prost(int32, repeated, tag="1")]
    pub xs: ::prost::alloc::vec::Vec<i32>,
}
"#;
        let actual = transform(original);
        assert!(
            actual.contains("#[femtopb(int32, packed, tag = 1)]"),
            "{actual}"
        );
        assert!(
            actual.contains("::femtopb::packed::Packed<'a, i32, ::femtopb::item_encoding::Int32>"),
            "{actual}"
        );
    }

    #[test]
    fn packed_false_before_repeated_still_yields_a_repeated_field() {
        // prost-build writes `repeated` before `packed="false"`, but nothing about the attribute
        // format guarantees that order. Under the old string-patching approach the label and the
        // `is_repeated`/`is_packed` flags were updated independently, so a reordered attribute
        // produced a `packed` label on a field typed as `Repeated` — code that does not compile.
        // The label is now derived from the same state the field type is, so order cannot matter.
        let original = r#"
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct M {
    #[prost(int32, packed="false", repeated, tag="1")]
    pub xs: ::prost::alloc::vec::Vec<i32>,
}
"#;
        let actual = transform(original);
        assert!(actual.contains("#[femtopb(int32, repeated, tag = 1)]"), "{actual}");
        assert!(!actual.contains("packed"), "{actual}");
        assert!(
            actual.contains("::femtopb::repeated::Repeated<'a, i32, ::femtopb::item_encoding::Int32>"),
            "{actual}"
        );
    }

    #[test]
    fn packed_false_on_a_repeated_enum_yields_a_repeated_field() {
        // Enums are packable as far as prost is concerned, so it emits `packed="false"` for an
        // explicitly-unpacked repeated enum just as it does for scalars. Both the label and the
        // field type must follow.
        let original = r#"
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct M {
    #[prost(enumeration="Color", repeated, packed="false", tag="1")]
    pub xs: ::prost::alloc::vec::Vec<i32>,
}
"#;
        let actual = transform(original);
        assert!(
            actual.contains("#[femtopb(enumeration, repeated, tag = 1)]"),
            "{actual}"
        );
        assert!(actual.contains("::femtopb::repeated::Repeated"), "{actual}");
        assert!(!actual.contains("::femtopb::packed::Packed"), "{actual}");
    }

    #[test]
    fn transform_explicit_packed_true_becomes_packed() {
        // Exercises the `packed="true"` string-handling branch. The scalar is already packable, so
        // `repeated` and `packed="true"` agree; the label is written once, from the final state of
        // the metadata, rather than once per argument that touched it.
        let original = r#"
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct M {
    #[prost(sfixed64, repeated, packed="true", tag="1")]
    pub xs: ::prost::alloc::vec::Vec<i64>,
}
"#;
        let actual = transform(original);
        assert!(
            actual.contains("#[femtopb(sfixed64, packed, tag = 1)]"),
            "{actual}"
        );
        assert!(!actual.contains("repeated"), "{actual}");
        assert!(
            actual
                .contains("::femtopb::packed::Packed<'a, i64, ::femtopb::item_encoding::SFixed64>"),
            "{actual}"
        );
    }

    #[test]
    fn transform_repeated_enum_defaults_to_packed_enumvalue() {
        // Enumerations are packable, so a repeated enum with no explicit `packed` lowers to
        // `Packed` over `EnumValue`. The only active enum-repeated coverage pins `packed="false"`.
        let original = r#"
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct M {
    #[prost(enumeration="Color", repeated, tag="1")]
    pub colors: ::prost::alloc::vec::Vec<i32>,
}
"#;
        let actual = transform(original);
        assert!(
            actual.contains("#[femtopb(enumeration, packed, tag = 1)]"),
            "{actual}"
        );
        assert!(
            actual.contains(
                "::femtopb::packed::Packed<\n        'a,\n        ::femtopb::enumeration::EnumValue<Color>,\n        ::femtopb::item_encoding::Enum<Color>,\n    >"
            ) || actual.contains(
                "::femtopb::packed::Packed<'a, ::femtopb::enumeration::EnumValue<Color>, ::femtopb::item_encoding::Enum<Color>>"
            ),
            "{actual}"
        );
    }

    #[test]
    fn transform_repeated_message_stays_repeated() {
        // Messages are not packable, so a repeated message lowers to `Repeated` over the `Message`
        // item encoding. Only the `#[ignore]`d reference test touches this otherwise.
        let original = r#"
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct M {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Item>,
}
"#;
        let actual = transform(original);
        assert!(
            actual.contains("#[femtopb(message, repeated, tag = 1)]"),
            "{actual}"
        );
        assert!(
            actual.contains("::femtopb::repeated::Repeated<"),
            "{actual}"
        );
        assert!(
            actual.contains("::femtopb::item_encoding::Message<'a, Item<'a>>"),
            "{actual}"
        );
    }
}
