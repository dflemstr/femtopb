use crate::field;

#[derive(Clone)]
pub struct Field {
    pub kind: field::Kind,
    pub tag: u32,
    pub default_value: Option<syn::Expr>,
}

impl Field {
    pub fn new(span: proc_macro2::Span, spec: &field::Spec) -> syn::Result<Option<Self>> {
        if spec.enumeration {
            let tag = spec.tag.ok_or_else(|| {
                syn::Error::new(
                    span,
                    "A field of type `enumeration` must have a `tag` attribute",
                )
            })?;
            let kind = field::Kind::from_spec(span, spec)?;
            let default_value = spec.default.clone();

            Ok(Some(Self {
                kind,
                tag,
                default_value,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn encode_raw_block(
        &self,
        field: &proc_macro2::TokenStream,
        cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let tag = self.tag;
        let (func, needs_default) = match self.kind {
            field::Kind::Plain | field::Kind::Required => (quote::quote!(encode), true),
            field::Kind::Optional => (quote::quote!(encode_optional), true),
            field::Kind::Repeated => (quote::quote!(encode_repeated), false),
            field::Kind::Packed => (quote::quote!(encode_packed), false),
        };

        let default = if let Some(d) = self.default_value.as_ref() {
            quote::quote!(::core::option::Option::Some(#d))
        } else {
            quote::quote!(::core::option::Option::None)
        };

        if needs_default {
            Ok(quote::quote! {
                ::femtopb::runtime::enumeration::#func(#tag, #field, #default, #cursor);
            })
        } else {
            Ok(quote::quote! {
                ::femtopb::runtime::enumeration::#func(#tag, #field, #cursor);
            })
        }
    }

    pub fn encoded_len_expr(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let tag = self.tag;
        let (func, needs_default) = match self.kind {
            field::Kind::Plain | field::Kind::Required => (quote::quote!(encoded_len), true),
            field::Kind::Optional => (quote::quote!(encoded_len_optional), true),
            field::Kind::Repeated => (quote::quote!(encoded_len_repeated), false),
            field::Kind::Packed => (quote::quote!(encoded_len_packed), false),
        };

        let default = if let Some(d) = self.default_value.as_ref() {
            quote::quote!(::core::option::Option::Some(#d))
        } else {
            quote::quote!(::core::option::Option::None)
        };

        if needs_default {
            Ok(quote::quote! {
                ::femtopb::runtime::enumeration::#func(#tag, #field, #default)
            })
        } else {
            Ok(quote::quote! {
                ::femtopb::runtime::enumeration::#func(#tag, #field)
            })
        }
    }

    pub fn decode_match_arm(
        &self,
        field: &proc_macro2::TokenStream,
        wire_type: &proc_macro2::TokenStream,
        msg_buf: &proc_macro2::TokenStream,
        cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let tag = self.tag;
        let decode_raw_block = self.decode_raw_block(field, wire_type, msg_buf, cursor)?;
        Ok(quote::quote! {
            #tag => {
                #decode_raw_block
            },
        })
    }

    pub fn decode_raw_block(
        &self,
        field: &proc_macro2::TokenStream,
        wire_type: &proc_macro2::TokenStream,
        msg_buf: &proc_macro2::TokenStream,
        cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let tag = self.tag;
        let func = match self.kind {
            field::Kind::Plain | field::Kind::Required => quote::quote!(decode),
            field::Kind::Optional => quote::quote!(decode_optional),
            field::Kind::Repeated => quote::quote!(decode_repeated),
            field::Kind::Packed => quote::quote!(decode_packed),
        };
        Ok(quote::quote! {
            ::femtopb::runtime::enumeration::#func(#tag, #wire_type, #msg_buf, #cursor, &mut #field)?;
        })
    }

    pub fn clear_block(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let func = match self.kind {
            field::Kind::Plain | field::Kind::Required => quote::quote!(clear),
            field::Kind::Optional => quote::quote!(clear_optional),
            field::Kind::Repeated => quote::quote!(clear_repeated),
            field::Kind::Packed => quote::quote!(clear_packed),
        };
        Ok(quote::quote! {
            ::femtopb::runtime::enumeration::#func(&mut #field);
        })
    }

    pub fn default_expr(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self.kind {
            field::Kind::Plain | field::Kind::Required => {
                if let Some(default) = self.default_value.as_ref() {
                    Ok(quote::quote!(::femtopb::enumeration::EnumValue::Known(#default)))
                } else {
                    Ok(quote::quote!(::core::default::Default::default()))
                }
            }
            field::Kind::Optional => Ok(quote::quote!(::core::option::Option::None)),
            field::Kind::Repeated => Ok(quote::quote!(::femtopb::repeated::Repeated::empty())),
            field::Kind::Packed => Ok(quote::quote!(::femtopb::packed::Packed::empty())),
        }
    }
}
