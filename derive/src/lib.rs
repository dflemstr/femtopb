use crate::field::Spec;
use syn::spanned::Spanned;

mod field;

struct RawFields {
    fields: Vec<syn::Field>,
}

#[proc_macro_derive(Message, attributes(femtopb))]
pub fn derive_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    try_derive_message(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Enumeration, attributes(femtopb))]
pub fn derive_enumeration(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    try_derive_enumeration(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Oneof, attributes(femtopb))]
pub fn derive_oneof(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    try_derive_oneof(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn try_derive_message(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = input.ident;

    let lt = get_first_lifetime(&input.generics, "::femtopb::Message")?;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let RawFields { fields } = collect_raw_fields(ident.span(), input.data)?;
    let mut fields = parse_fields(fields)?;
    fields.sort_by_key(|(_, field)| field.ord_key());

    let known_tags = fields
        .iter()
        .flat_map(|(_, f)| f.tags())
        .collect::<Vec<_>>();

    let decode_remaining = quote::quote!(&mut remaining);
    let encode_cursor = quote::quote!(cursor);
    let wire_type = quote::quote!(wire_type);
    // The whole message buffer, and the buffer position at the start of the current field (before
    // its key). The lazily-parsed field types (`repeated`/`packed`) and `unknown_fields` record
    // their span as offsets of `field_start` into `msg_buf`, so re-iterating/re-encoding them skips
    // the fields that precede (and, once narrowed, follow) their occurrences.
    let msg_buf = quote::quote!(msg_buf);
    let field_start = quote::quote!(field_start);

    let encode_raw_blocks = fields
        .iter()
        .map(|(id, f)| f.encode_raw_block(&quote::quote!(self.#id), &encode_cursor))
        .collect::<syn::Result<Vec<_>>>()?;
    let encoded_len_exprs = fields
        .iter()
        .flat_map(|(id, f)| f.encoded_len_expr(&quote::quote!(self.#id)).transpose())
        .collect::<syn::Result<Vec<_>>>()?;
    let decode_match_arms = fields
        .iter()
        .map(|(id, f)| {
            f.decode_match_arm(
                &quote::quote!(tag),
                &quote::quote!(value.#id),
                &wire_type,
                &msg_buf,
                &field_start,
                &decode_remaining,
                &known_tags,
            )
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let clear_blocks = fields
        .iter()
        .map(|(id, f)| f.clear_block(&quote::quote!(self.#id)))
        .collect::<syn::Result<Vec<_>>>()?;
    let default_fields = fields
        .iter()
        .map(|(id, f)| {
            let expr = f.default_expr()?;
            Ok(quote::quote!(#id: #expr,))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let encoded_len_body = if encoded_len_exprs.is_empty() {
        quote::quote!(0)
    } else {
        quote::quote!(#(#encoded_len_exprs)+*)
    };

    let decode_body = if fields.is_empty() {
        quote::quote! {}
    } else {
        quote::quote! {
            let mut remaining = msg_buf;
            while !remaining.is_empty() {
                // The buffer as it stands before this field's key; the lazily-parsed and
                // unknown-fields decoders use it to anchor their retained slice at this field.
                let field_start = remaining;
                let (tag, wire_type) = ::femtopb::encoding::decode_key(&mut remaining)?;
                match tag {
                    #(#decode_match_arms)*
                }
            }
        }
    };

    Ok(quote::quote! {
        // The generated impl reads every field — including any the user marked `#[deprecated]`, which
        // it must to round-trip them — and is machine output rather than idiomatic hand-written code,
        // so suppress lints on it (matching what `femtopb-build` does for the generated structs).
        #[allow(clippy::all, deprecated)]
        #[automatically_derived]
        impl #impl_generics ::femtopb::message::Message<#lt> for #ident #ty_generics #where_clause {
            fn encode_raw(&self, cursor: &mut &mut [u8]) {
                #(#encode_raw_blocks)*
            }

            fn encoded_len(&self) -> usize {
                #encoded_len_body
            }

            fn decode(msg_buf: &#lt [u8]) -> Result<Self, ::femtopb::error::DecodeError> where Self: Sized {
                let mut value = <Self as ::core::default::Default>::default();

                #decode_body

                Ok(value)
            }

            fn clear(&mut self) {
                #(#clear_blocks)*
            }
        }

        #[allow(clippy::all, deprecated)]
        #[automatically_derived]
        impl #impl_generics ::core::default::Default for #ident #ty_generics #where_clause {
            fn default() -> Self {
                Self {
                    #(#default_fields)*
                }
            }
        }
    })
}

fn try_derive_enumeration(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let variants = match input.data {
        syn::Data::Enum(data_enum) => data_enum.variants.into_iter(),
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "`::femtopb::Enumeration` can only be derived for an `enum`",
            ))
        }
    };

    let variant_numbers = variants
        .map(|v| {
            let span = v.span();
            let (_, discriminant) = v.discriminant.ok_or(syn::Error::new(
                span,
                "Every enum variant must have a discriminant (ie. ` = <some int>`)",
            ))?;
            Ok((v.ident, discriminant))
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    let encode_match_arms = variant_numbers
        .iter()
        .map(|(ident, discriminant)| quote::quote!(Self::#ident => #discriminant,));
    let decode_match_arms = variant_numbers.iter().map(|(ident, discriminant)| quote::quote!(#discriminant => ::femtopb::enumeration::EnumValue::Known(Self::#ident),));

    Ok(quote::quote! {
        #[allow(clippy::all, deprecated)]
        #[automatically_derived]
        impl #impl_generics ::femtopb::enumeration::Enumeration for #ident #ty_generics #where_clause {
            fn encode(&self) -> i32 {
                match *self {
                    #(#encode_match_arms)*
                }
            }
            fn decode(value: i32) -> ::femtopb::enumeration::EnumValue<Self> {
                match value {
                    #(#decode_match_arms)*
                    n => ::femtopb::enumeration::EnumValue::Unknown(n),
                }
            }
        }
    })
}

struct OneofVariant {
    ident: syn::Ident,
    field: field::Field,
}

fn try_derive_oneof(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;

    // The `Oneof` trait is parameterised over the lifetime of the buffer being decoded. If the enum
    // already declares a lifetime (because it has borrowing variants such as `&'a str`), reuse it.
    // Otherwise synthesise a fresh one that appears only in the impl header, so that a oneof made up
    // solely of scalar variants — which has no reason to carry a lifetime — can still derive
    // `Oneof` without the enum tripping "unused lifetime parameter" (E0392).
    let mut impl_generics_src = input.generics.clone();
    let lt = match first_lifetime(&input.generics) {
        Some(lt) => lt.clone(),
        None => {
            let lt = syn::Lifetime::new("'femtopb", proc_macro2::Span::call_site());
            impl_generics_src.params.insert(
                0,
                syn::GenericParam::Lifetime(syn::LifetimeParam::new(lt.clone())),
            );
            lt
        }
    };
    let (impl_generics, _, _) = impl_generics_src.split_for_impl();
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();

    let variants = match input.data {
        syn::Data::Enum(data_enum) => data_enum.variants.into_iter().map(|variant| {
            let variant_span = variant.span();
            let ident = variant.ident;
            let mut fields = variant.fields.into_iter();

            if let Some(field) = fields.next() {
                if let Some(field) = fields.next() {
                    Err(syn::Error::new(field.span(), "`::femtopb::Oneof` cannot be derived for enums where a variant has more than one field"))
                } else {
                    let (span, mut spec) = Spec::find_attr_and_parse(field.span(), &variant.attrs)?;
                    // Implicit for oneoffs
                    spec.required = true;
                    let field = field::Field::from_spec(span, spec)?;
                    Ok(field.map(|f| OneofVariant { ident, field: f}))
                }
            } else {
                Err(syn::Error::new(variant_span, "This variant needs a field to contain the `::femtopb::Oneof` data"))
            }
        }).flat_map(|r| r.transpose()).collect::<syn::Result<Vec<_>>>()?,
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "`::femtopb::Oneof` can only be derived for an `enum`",
            ))
        }
    };

    check_oneof_tags_are_unique(&variants)?;

    let encode_match_arms = variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let encode_raw_block = v
                .field
                .encode_raw_block(&quote::quote!(*v), &quote::quote!(cursor))?;
            Ok(quote::quote! {
                Self::#ident(ref v) => {
                    #encode_raw_block
                },
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let encoded_len_match_arms = variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let encoded_len_expr = v.field.encoded_len_expr(&quote::quote!(*v))?;
            Ok(quote::quote! {
                Self::#ident(ref v) => {
                    #encoded_len_expr
                },
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let decode_match_arms = variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let tag = if let &[tag] = v.field.tags().into_iter().collect::<Vec<_>>().as_slice() {
                tag
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    "This variant has multiple field `tags` defined, which is not supported",
                ));
            };
            // A oneof's variants are never lazily-parsed, so `msg_buf`/`field_start` are ignored;
            // `O::decode` only has the one buffer parameter, so pass it for both positions.
            let decode_raw_block = v.field.decode_raw_block(
                &quote::quote!(tag),
                &quote::quote!(v),
                &quote::quote!(wire_type),
                &quote::quote!(msg_buf),
                &quote::quote!(msg_buf),
                &quote::quote!(cursor),
                &[],
            )?;

            Ok(quote::quote! {
                #tag => {
                    let mut v = ::core::default::Default::default();
                    #decode_raw_block
                    ::core::result::Result::Ok(Self::#ident(v))
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    // TODO: there's potential lifetime shadowing if the `fn decode` lifetime is named the same as
    // a parent lifetime. Can we generate a fresh lifetime name here?
    Ok(quote::quote! {
        #[allow(clippy::all, deprecated)]
        #[automatically_derived]
        impl #impl_generics ::femtopb::oneof::Oneof<#lt> for #ident #ty_generics #where_clause {
            fn encode(&self, cursor: &mut &mut [u8]) {
                match *self {
                    #(#encode_match_arms)*
                    _ => {},
                }
            }
            fn encoded_len(&self) -> usize {
                match *self {
                    #(#encoded_len_match_arms)*
                    _ => 0,
                }
            }
            fn decode(tag: u32, wire_type: ::femtopb::encoding::WireType, msg_buf: &#lt [u8], cursor: &mut &#lt [u8]) -> ::core::result::Result<Self, ::femtopb::error::DecodeError> {
                match tag {
                    #(#decode_match_arms)*
                    n => ::core::result::Result::Err(::femtopb::error::DecodeError::UnexpectedTagValue(n))
                }
            }
        }
    })
}

/// Rejects a oneof in which two variants claim the same tag.
///
/// `Oneof::decode` dispatches on the tag, so a repeated tag makes the later variant unreachable: a
/// value encoded as that variant decodes back as the earlier one.
fn check_oneof_tags_are_unique(variants: &[OneofVariant]) -> syn::Result<()> {
    let mut seen = std::collections::HashSet::new();
    for variant in variants {
        for tag in variant.field.tags() {
            if !seen.insert(tag) {
                return Err(syn::Error::new(
                    variant.ident.span(),
                    format_args!("tag `{tag}` is used by more than one variant of this oneof"),
                ));
            }
        }
    }
    Ok(())
}

fn first_lifetime(generics: &syn::Generics) -> Option<&syn::Lifetime> {
    generics.params.iter().find_map(|p| {
        if let syn::GenericParam::Lifetime(syn::LifetimeParam { lifetime, .. }) = p {
            Some(lifetime)
        } else {
            None
        }
    })
}

fn get_first_lifetime<'a>(
    generics: &'a syn::Generics,
    derived_trait: &str,
) -> syn::Result<&'a syn::Lifetime> {
    first_lifetime(generics).ok_or(syn::Error::new(
        proc_macro2::Span::call_site(),
        format!(
            "`{derived_trait}` must only be derived for structs which have a lifetime parameter"
        ),
    ))
}

fn collect_raw_fields(span: proc_macro2::Span, data: syn::Data) -> syn::Result<RawFields> {
    match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => Ok(RawFields {
            fields: named.into_iter().collect(),
        }),
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        }) => Ok(RawFields {
            fields: unnamed.into_iter().collect(),
        }),
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => Ok(RawFields { fields: Vec::new() }),
        syn::Data::Enum(_) => Err(syn::Error::new(
            span,
            "`Message` can not be derived for an `enum`",
        )),
        syn::Data::Union(_) => Err(syn::Error::new(
            span,
            "`Message` can not be derived for a `union`",
        )),
    }
}

fn parse_fields(
    fields: Vec<syn::Field>,
) -> syn::Result<Vec<(proc_macro2::TokenStream, field::Field)>> {
    use syn::spanned::Spanned as _;

    let fields = fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            let span = field.span();
            let id = field
                .ident
                .as_ref()
                .map(|x| quote::quote!(#x))
                .unwrap_or_else(|| {
                    let index = u32::try_from(i).unwrap();
                    let index = syn::Index { index, span };
                    quote::quote!(#index)
                });
            let parsed = field::Field::new(span, field.attrs)?;
            Ok(parsed.map(|f| (span, id, f)))
        })
        .flat_map(|r: syn::Result<_>| r.transpose())
        .collect::<syn::Result<Vec<_>>>()?;

    check_tags_are_unique(&fields)?;
    check_at_most_one_unknown_fields(&fields)?;

    Ok(fields.into_iter().map(|(_, id, f)| (id, f)).collect())
}

type ParsedField = (proc_macro2::Span, proc_macro2::TokenStream, field::Field);

/// Rejects a message in which two fields claim the same tag.
///
/// The generated decoder is a `match` on the tag, so a repeated tag makes every arm after the first
/// dead: the later field silently never decodes, and both fields still encode, producing a message
/// that does not round-trip. Nothing in the generated code warns about this, so catch it here.
fn check_tags_are_unique(fields: &[ParsedField]) -> syn::Result<()> {
    let mut seen = std::collections::HashMap::new();
    for (span, _, f) in fields {
        for tag in f.tags() {
            if let Some(_previous) = seen.insert(tag, *span) {
                return Err(syn::Error::new(
                    *span,
                    format_args!("tag `{tag}` is used by more than one field of this message"),
                ));
            }
        }
    }
    Ok(())
}

/// Rejects a message that declares more than one `#[femtopb(unknown_fields)]` field.
///
/// The field backs the catch-all arm of the generated decoder, so only the first one would ever be
/// populated; any further ones would silently stay empty and drop the fields they claim to preserve.
fn check_at_most_one_unknown_fields(fields: &[ParsedField]) -> syn::Result<()> {
    let mut seen = false;
    for (span, _, f) in fields {
        if matches!(f, field::Field::UnknownFields(_)) {
            if seen {
                return Err(syn::Error::new(
                    *span,
                    "a message can have at most one `#[femtopb(unknown_fields)]` field",
                ));
            }
            seen = true;
        }
    }
    Ok(())
}
