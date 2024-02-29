use crate::field;

#[derive(Clone)]
pub struct Field {
    pub label: field::Label,
    pub tag: u32,
}

impl Field {
    pub fn new(span: proc_macro2::Span, spec: &field::Spec) -> syn::Result<Option<Self>> {
        if spec.message {
            let tag = spec.tag.ok_or_else(|| {
                syn::Error::new(
                    span,
                    "A field of type `message` must have a `tag` attribute",
                )
            })?;
            let label = field::Label::from_spec(span, spec)?.unwrap_or(field::Label::Optional);

            Ok(Some(Self { tag, label }))
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
        let func = match self.label {
            field::Label::Required => quote::quote!(encode),
            field::Label::Optional => quote::quote!(encode_optional),
            field::Label::Repeated => quote::quote!(encode_repeated),
        };
        Ok(quote::quote! {
            ::femtopb::runtime::message::#func(#tag, &#field, #cursor);
        })
    }

    pub fn encoded_len_expr(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let tag = self.tag;
        let func = match self.label {
            field::Label::Required => quote::quote!(encoded_len),
            field::Label::Optional => quote::quote!(encoded_len_optional),
            field::Label::Repeated => quote::quote!(encoded_len_repeated),
        };
        Ok(quote::quote! {
            ::femtopb::runtime::message::#func(#tag, &#field)
        })
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
        let func = match self.label {
            field::Label::Required => quote::quote!(decode),
            field::Label::Optional => quote::quote!(decode_optional),
            field::Label::Repeated => quote::quote!(decode_repeated),
        };
        Ok(quote::quote! {
            ::femtopb::runtime::message::#func(#tag, #wire_type, #msg_buf, #cursor, &mut #field)?;
        })
    }

    pub fn clear_block(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let tag = self.tag;
        let func = match self.label {
            field::Label::Required => quote::quote!(clear),
            field::Label::Optional => quote::quote!(clear_optional),
            field::Label::Repeated => quote::quote!(clear_repeated),
        };
        Ok(quote::quote! {
            ::femtopb::runtime::message::#func(#tag, &mut #field);
        })
    }

    pub fn default_expr(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self.label {
            field::Label::Required => Ok(quote::quote!(::core::default::Default::default())),
            field::Label::Optional => Ok(quote::quote!(::core::option::Option::None)),
            field::Label::Repeated => Ok(quote::quote!(::femtopb::repeated::Repeated::empty())),
        }
    }
}
