use crate::field;

#[derive(Clone)]
pub struct Field {
    pub tags: Vec<u32>,
}

impl Field {
    pub fn new(_span: proc_macro2::Span, spec: &field::Spec) -> syn::Result<Option<Self>> {
        if spec.oneof {
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
        cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let tags = &self.tags;
        if tags.is_empty() {
            Ok(quote::quote!())
        } else {
            let decode_raw_block =
                self.decode_raw_block(matched_tag, field, wire_type, msg_buf, cursor)?;
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
        cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote::quote! {
            ::femtopb::runtime::oneof::decode(#matched_tag, #wire_type, #msg_buf, #cursor, &mut #field)?;
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
