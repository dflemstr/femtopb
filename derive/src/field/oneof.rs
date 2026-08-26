use crate::field;

#[derive(Clone)]
pub struct Field {
    pub tags: Vec<u32>,
}

impl Field {
    pub fn new(span: proc_macro2::Span, spec: &field::Spec) -> syn::Result<Option<Self>> {
        if spec.oneof {
            // Without tags there is nothing for the generated decoder to match on, so the field
            // would encode but silently never decode: its bytes would be swept into the message's
            // unknown fields and the oneof would always come back as `None`.
            if spec.tags.is_empty() {
                return Err(syn::Error::new(
                    span,
                    "A field of type `oneof` must list the tags of its variants in a `tags` \
                     attribute (e.g. `#[femtopb(oneof, tags = [1, 2])]`)",
                ));
            }
            let tags = spec.tags.clone();
            Ok(Some(Self { tags }))
        } else {
            Ok(None)
        }
    }

    pub fn encode_raw_block(
        &self,
        field: &proc_macro2::TokenStream,
        cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote::quote! {
            ::femtopb::runtime::oneof::encode(&#field, #cursor);
        })
    }

    pub fn encoded_len_expr(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote::quote! {
            ::femtopb::runtime::oneof::encoded_len(&#field)
        })
    }

    pub fn decode_match_arm(
        &self,
        matched_tag: &proc_macro2::TokenStream,
        field: &proc_macro2::TokenStream,
        wire_type: &proc_macro2::TokenStream,
        msg_buf: &proc_macro2::TokenStream,
        field_start: &proc_macro2::TokenStream,
        cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let tags = &self.tags;
        if tags.is_empty() {
            Ok(quote::quote!())
        } else {
            let decode_raw_block =
                self.decode_raw_block(matched_tag, field, wire_type, msg_buf, field_start, cursor)?;
            Ok(quote::quote! {
                #(#tags)|* => {
                    #decode_raw_block
                },
            })
        }
    }

    pub fn decode_raw_block(
        &self,
        matched_tag: &proc_macro2::TokenStream,
        field: &proc_macro2::TokenStream,
        wire_type: &proc_macro2::TokenStream,
        msg_buf: &proc_macro2::TokenStream,
        field_start: &proc_macro2::TokenStream,
        cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        // Uniform decoder signature (whole `#msg_buf` plus this field's `#field_start`); the oneof
        // decoder ignores both and dispatches on the tag.
        Ok(quote::quote! {
            ::femtopb::runtime::oneof::decode(#matched_tag, #wire_type, #msg_buf, #field_start, #cursor, &mut #field)?;
        })
    }

    pub fn clear_block(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote::quote! {
            ::femtopb::runtime::oneof::clear(&mut #field);
        })
    }

    pub fn default_expr(&self) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote::quote!(::core::option::Option::None))
    }
}
