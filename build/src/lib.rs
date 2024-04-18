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
        config.format(false).bytes(&["."]).prost_path("::femtopb");

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

    for item in &mut file.items {
        transform_item(item);
    }

    prettyplease::unparse(&file)
}

fn transform_item(item: &mut syn::Item) {
    match *item {
        syn::Item::Struct(ref mut struct_item) if has_message_derive(struct_item) => {
            struct_item
                .generics
                .params
                .push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
                    syn::Lifetime::new("'a", proc_macro2::Span::call_site()),
                )));

            for field in &mut struct_item.fields {
                transform_field(field);
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
            enum_item
                .generics
                .params
                .push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
                    syn::Lifetime::new("'a", proc_macro2::Span::call_site()),
                )));

            for variant in &mut enum_item.variants {
                transform_variant(variant);
            }

            enum_item.variants.push(syn::parse_quote! {
                #[femtopb(phantom)]
                _Phantom(::core::marker::PhantomData<&'a ()>)
            });
        }
        syn::Item::Mod(ref mut item_mod) => {
            if let Some(ref mut content) = item_mod.content {
                for mod_item in &mut content.1 {
                    transform_item(mod_item);
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
        let mut new_attr = "femtopb(".to_string();
        let mut needs_separator = false;

        let mut add_separator = |str: &mut String| {
            if needs_separator {
                str.push_str(", ");
            }
            needs_separator = true;
        };

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
                add_separator(&mut new_attr);
                new_attr.push_str(&name);
                metadata.is_scalar = Some(name);
            } else if path.is_ident("optional")
                || path.is_ident("required")
                || path.is_ident("map")
                || path.is_ident("hash_map")
                || path.is_ident("btree_map")
            {
                add_separator(&mut new_attr);
                new_attr.push_str(&path.segments[0].ident.to_string());
            } else if path.is_ident("boxed") {
                add_separator(&mut new_attr);
                new_attr.push_str("deferred");
            } else if path.is_ident("message") || path.is_ident("group") {
                metadata.is_message = true;
                add_separator(&mut new_attr);
                new_attr.push_str(&path.segments[0].ident.to_string());
            } else if path.is_ident("repeated") {
                add_separator(&mut new_attr);
                let packed = if let Some(scalar) = &metadata.is_scalar {
                    // Assume packed, change back later if we encounter `packed="false"` from
                    // prost
                    can_pack_scalar(scalar)
                } else {
                    metadata.is_enum.is_some()
                };

                if packed {
                    metadata.is_packed = true;
                    new_attr.push_str("packed");
                } else {
                    metadata.is_repeated = true;
                    new_attr.push_str("repeated");
                }
            } else if path.is_ident("packed") {
                let name_value = meta.require_name_value().unwrap();
                match &name_value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(str),
                        ..
                    }) => match str.value().as_str() {
                        "true" => {
                            metadata.is_repeated = false;
                            metadata.is_packed = true;
                            if new_attr.contains("repeated") {
                                new_attr = new_attr.replace("repeated", "packed");
                            } else {
                                new_attr.push_str("packed");
                            }
                        }
                        "false" => {
                            metadata.is_repeated = true;
                            metadata.is_packed = false;
                            if new_attr.contains("packed") {
                                new_attr = new_attr.replace("packed", "repeated");
                            } else {
                                new_attr.push_str("packed");
                            }
                        }
                        _ => unreachable!(),
                    },
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
                        add_separator(&mut new_attr);
                        new_attr.push_str(&path.segments[0].ident.to_string());
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
                        add_separator(&mut new_attr);
                        new_attr.push_str(&path.segments[0].ident.to_string());
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
                        add_separator(&mut new_attr);
                        new_attr.push_str("tag = ");
                        new_attr.push_str(str.value().as_str());
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
                        add_separator(&mut new_attr);
                        new_attr.push_str("tags = [");
                        new_attr.push_str(str.value().as_str());
                        new_attr.push_str("]");
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
                        add_separator(&mut new_attr);
                        new_attr.push_str("default = ");
                        if metadata.is_scalar.as_deref() == Some("string") {
                            new_attr.push_str(&format!("{:?}", str.value().as_str()));
                        } else if let Some(e) = metadata.is_enum.as_ref() {
                            let ident: syn::Ident = syn::parse_str(str.value().as_str()).unwrap();
                            new_attr.push_str(&quote::quote!(#e::#ident).to_string());
                        } else {
                            new_attr.push_str(str.value().as_str());
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                panic!("unhandled prost attr: {:?}", path.get_ident().unwrap());
            }
        }
        new_attr.push_str(")");
        let new_meta: syn::Meta = syn::parse_str(&new_attr).unwrap();
        attr.meta = new_meta;
    }
}

fn transform_field(field: &mut syn::Field) {
    let mut metadata = FieldMetadata::default();
    for attr in &mut field.attrs {
        transform_prost_attr(attr, &mut metadata);
    }
    transform_field_type(&mut field.ty, &metadata);
}

fn transform_variant(variant: &mut syn::Variant) {
    let mut metadata = FieldMetadata::default();
    for attr in &mut variant.attrs {
        transform_prost_attr(attr, &mut metadata);
    }
    transform_field_type(&mut variant.fields.iter_mut().next().unwrap().ty, &metadata);
}

fn transform_field_type(ty: &mut syn::Type, metadata: &FieldMetadata) {
    match ty {
        syn::Type::Path(syn::TypePath { ref mut path, .. }) => {
            // Check for option/vec/box before is_enum/is_message to handle the optional/repeated
            // messages/enums
            if has_same_path_idents(path, "::core::option::Option") {
                let generic_segment = path.segments.last_mut().unwrap();
                transform_field_type(get_single_generic_arg(generic_segment), metadata);
            } else if has_same_path_idents(path, "::femtopb::alloc::boxed::Box")
                || has_same_path_idents(path, "::prost::alloc::boxed::Box")
            {
                let generic_segment = path.segments.last_mut().unwrap();
                let inner_ty = get_single_generic_arg(generic_segment);
                transform_field_type(inner_ty, metadata);
                *ty = syn::parse2(quote::quote!(::femtopb::deferred::Deferred<'a, #inner_ty>))
                    .unwrap();
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
                transform_field_type(inner_ty, metadata);
                *ty =
                    syn::parse2(quote::quote!(#base_type<'a, #inner_ty, #item_encoding>)).unwrap();
            } else if metadata.is_message || metadata.is_oneof.is_some() {
                let generic_segment = path.segments.last_mut().unwrap();
                let ident = &generic_segment.ident;
                *generic_segment = syn::parse2(quote::quote!(#ident<'a>)).unwrap()
            } else if let Some(enum_ty) = &metadata.is_enum {
                *ty = syn::parse2(quote::quote!(::femtopb::enumeration::EnumValue<#enum_ty>))
                    .unwrap();
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
        _ => {}
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
        && path_segments(&path) == path_segments(&parsed_other)
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
        assert_eq!(actual.trim(), expected.trim());
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
        assert_eq!(actual.trim(), expected.trim());
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
        assert_eq!(actual.trim(), expected.trim());
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
        assert_eq!(actual.trim(), expected.trim());
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
        assert_eq!(actual.trim(), expected.trim());
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
        assert_eq!(actual.trim(), expected.trim());
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
        assert_eq!(actual.trim(), expected.trim());
    }
}
