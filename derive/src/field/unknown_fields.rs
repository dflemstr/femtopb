use crate::field;

pub struct Field;

impl Field {
    pub fn new(_span: proc_macro2::Span, spec: &field::Spec) -> syn::Result<Option<Self>> {
        if spec.unknown_fields {
            Ok(Some(Field))
        } else {
            Ok(None)
        }
    }

    pub fn encode_raw_block(
        &self,
        _field: &proc_macro2::TokenStream,
        _cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote::quote!()) // TODO
    }

    pub fn decode_match_arm(
        &self,
        matched_tag: &proc_macro2::TokenStream,
        field: &proc_macro2::TokenStream,
        wire_type: &proc_macro2::TokenStream,
        msg_buf: &proc_macro2::TokenStream,
        cursor: &proc_macro2::TokenStream,
        known_tags: &[u32],
    ) -> syn::Result<proc_macro2::TokenStream> {
        let decode_raw_block =
            self.decode_raw_block(matched_tag, field, wire_type, msg_buf, cursor, known_tags)?;

        Ok(quote::quote! {
            _ => {
                #decode_raw_block
            },
        })
    }

    pub fn decode_raw_block(
        &self,
        matched_tag: &proc_macro2::TokenStream,
        field: &proc_macro2::TokenStream,
        wire_type: &proc_macro2::TokenStream,
        msg_buf: &proc_macro2::TokenStream,
        cursor: &proc_macro2::TokenStream,
        known_tags: &[u32],
    ) -> syn::Result<proc_macro2::TokenStream> {
        let known_tags = quote::quote!([#(#known_tags),*]);
        Ok(quote::quote! {
            ::femtopb::runtime::unknown_fields::decode(&#known_tags, #matched_tag, #wire_type, #msg_buf, #cursor, &mut #field)?;
        })
    }

    pub fn clear_block(
        &self,
        _field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote::quote!()) // TODO
    }

    pub fn default_expr(&self) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote::quote!(
            ::femtopb::unknown_fields::UnknownFields::empty()
        ))
    }
}
