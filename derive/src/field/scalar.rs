use crate::field;
use core::fmt;
use core::str;

#[derive(Clone)]
pub struct Field {
    pub type_: Type,
    pub kind: field::Kind,
    pub tag: u32,
    pub default_value: Option<DefaultValue>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Double,
    Float,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Fixed32,
    Fixed64,
    Sfixed32,
    Sfixed64,
    Bool,
    String,
    Bytes,
}

#[derive(Clone, Debug)]
pub enum DefaultValue {
    F64(f64),
    F32(f32),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
}

impl Field {
    pub fn new(span: proc_macro2::Span, spec: &field::Spec) -> syn::Result<Option<Self>> {
        if let &Some(type_) = &spec.scalar_type {
            let tag = spec
                .tag
                .ok_or_else(|| syn::Error::new(span, "no `tag` specified for this field"))?;
            let kind = field::Kind::from_spec(span, spec)?;
            let default_value = spec
                .default
                .as_ref()
                .map(|d| DefaultValue::parse(d, type_))
                .transpose()?;
            Ok(Some(Self {
                type_,
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
        let ty = syn::Ident::new(&self.type_.to_string(), proc_macro2::Span::call_site());
        let tag = self.tag;
        let default = self
            .default_value
            .as_ref()
            .unwrap_or(&DefaultValue::for_type(self.type_))
            .as_lit_expr();
        let (func, needs_default) = match self.kind {
            field::Kind::Plain | field::Kind::Required => (quote::quote!(encode), true),
            field::Kind::Optional => (quote::quote!(encode_optional), true),
            field::Kind::Repeated => (quote::quote!(encode_repeated), false),
            field::Kind::Packed => (quote::quote!(encode_packed), false),
        };

        if needs_default {
            Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::#func(#tag, #field, #default, #cursor);
            })
        } else {
            Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::#func(#tag, #field, #cursor);
            })
        }
    }

    pub fn encoded_len_expr(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let ty = syn::Ident::new(&self.type_.to_string(), proc_macro2::Span::call_site());
        let tag = self.tag;
        let default = self
            .default_value
            .as_ref()
            .unwrap_or(&DefaultValue::for_type(self.type_))
            .as_lit_expr();
        match self.kind {
            field::Kind::Plain | field::Kind::Required => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::encoded_len(#tag, #field, #default)
            }),
            field::Kind::Optional => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::encoded_len_optional(#tag, #field, #default)
            }),
            field::Kind::Repeated => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::encoded_len_repeated(#tag, #field)
            }),
            field::Kind::Packed => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::encoded_len_packed(#tag, #field)
            }),
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
        let ty = syn::Ident::new(&self.type_.to_string(), proc_macro2::Span::call_site());
        let tag = self.tag;
        match self.kind {
            field::Kind::Plain | field::Kind::Required => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::decode(#tag, #wire_type, #msg_buf, #cursor, &mut #field)?;
            }),
            field::Kind::Optional => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::decode_optional(#tag, #wire_type, #msg_buf, #cursor, &mut #field)?;
            }),
            field::Kind::Repeated => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::decode_repeated(#tag, #wire_type, #msg_buf, #cursor, &mut #field)?;
            }),
            field::Kind::Packed => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::decode_packed(#tag, #wire_type, #msg_buf, #cursor, &mut #field)?;
            }),
        }
    }

    pub fn clear_block(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let ty = syn::Ident::new(&self.type_.to_string(), proc_macro2::Span::call_site());
        match self.kind {
            field::Kind::Plain | field::Kind::Required => {
                let default = self
                    .default_value
                    .as_ref()
                    .unwrap_or(&DefaultValue::for_type(self.type_))
                    .as_lit_expr();
                Ok(quote::quote! {
                    ::femtopb::runtime::scalar::#ty::clear(&mut #field, #default);
                })
            }
            field::Kind::Optional => {
                let default = if let Some(ref v) = self.default_value {
                    let expr = v.as_lit_expr();
                    quote::quote!(::core::option::Option::Some(#expr))
                } else {
                    quote::quote!(::core::option::Option::None)
                };
                Ok(quote::quote! {
                    ::femtopb::runtime::scalar::#ty::clear_optional(&mut #field, #default);
                })
            }
            field::Kind::Repeated => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::clear_repeated(&mut #field);
            }),
            field::Kind::Packed => Ok(quote::quote! {
                ::femtopb::runtime::scalar::#ty::clear_packed(&mut #field);
            }),
        }
    }

    pub fn default_expr(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self.kind {
            field::Kind::Plain | field::Kind::Required => {
                let default = self
                    .default_value
                    .as_ref()
                    .unwrap_or(&DefaultValue::for_type(self.type_))
                    .as_lit_expr();
                Ok(quote::quote! {
                    #default
                })
            }
            field::Kind::Optional => {
                if let Some(default) = self.default_value.as_ref() {
                    let default = default.as_lit_expr();
                    Ok(quote::quote! {
                        ::core::option::Option::Some(#default)
                    })
                } else {
                    Ok(quote::quote! {
                        ::core::option::Option::None
                    })
                }
            }
            field::Kind::Repeated => Ok(quote::quote! {
                ::femtopb::repeated::Repeated::empty()
            }),
            field::Kind::Packed => Ok(quote::quote! {
                ::femtopb::packed::Packed::empty()
            }),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            Type::Double => "double",
            Type::Float => "float",
            Type::Int32 => "int32",
            Type::Int64 => "int64",
            Type::Uint32 => "uint32",
            Type::Uint64 => "uint64",
            Type::Sint32 => "sint32",
            Type::Sint64 => "sint64",
            Type::Fixed32 => "fixed32",
            Type::Fixed64 => "fixed64",
            Type::Sfixed32 => "sfixed32",
            Type::Sfixed64 => "sfixed64",
            Type::Bool => "bool",
            Type::String => "string",
            Type::Bytes => "bytes",
        })
    }
}

impl DefaultValue {
    fn parse(default: &syn::Expr, type_: Type) -> syn::Result<DefaultValue> {
        use syn::spanned::Spanned as _;
        match type_ {
            Type::Double => Ok(DefaultValue::F64(parse_float_literal(default, type_)?)),
            Type::Float => Ok(DefaultValue::F32(parse_float_literal(default, type_)?)),
            Type::Int32 => Ok(DefaultValue::I32(parse_int_literal(default, type_)?)),
            Type::Int64 => Ok(DefaultValue::I64(parse_int_literal(default, type_)?)),
            Type::Uint32 => Ok(DefaultValue::U32(parse_positive_int_literal(
                default, type_,
            )?)),
            Type::Uint64 => Ok(DefaultValue::U64(parse_positive_int_literal(
                default, type_,
            )?)),
            Type::Sint32 => Ok(DefaultValue::I32(parse_int_literal(default, type_)?)),
            Type::Sint64 => Ok(DefaultValue::I64(parse_int_literal(default, type_)?)),
            Type::Fixed32 => Ok(DefaultValue::U32(parse_positive_int_literal(
                default, type_,
            )?)),
            Type::Fixed64 => Ok(DefaultValue::U64(parse_positive_int_literal(
                default, type_,
            )?)),
            Type::Sfixed32 => Ok(DefaultValue::I32(parse_int_literal(default, type_)?)),
            Type::Sfixed64 => Ok(DefaultValue::I64(parse_int_literal(default, type_)?)),
            Type::Bool => {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Bool(bool),
                    ..
                }) = default
                {
                    Ok(DefaultValue::Bool(bool.value))
                } else {
                    Err(syn::Error::new(
                        default.span(),
                        format_args!(
                            "Expected a boolean default value for a field of type `{}`",
                            type_
                        ),
                    ))
                }
            }
            Type::String => {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(string),
                    ..
                }) = default
                {
                    Ok(DefaultValue::String(string.value()))
                } else {
                    Err(syn::Error::new(
                        default.span(),
                        format_args!(
                            "Expected a string default value for a field of type `{}`",
                            type_
                        ),
                    ))
                }
            }
            Type::Bytes => {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::ByteStr(bytes),
                    ..
                }) = default
                {
                    Ok(DefaultValue::Bytes(bytes.value()))
                } else {
                    Err(syn::Error::new(
                        default.span(),
                        format_args!(
                            "Expected a byte string default value for a field of type `{}`",
                            type_
                        ),
                    ))
                }
            }
        }
    }

    fn for_type(type_: Type) -> Self {
        match type_ {
            Type::Double => Self::F64(0.0),
            Type::Float => Self::F32(0.0),
            Type::Int32 | Type::Sint32 | Type::Sfixed32 => Self::I32(0),
            Type::Int64 | Type::Sint64 | Type::Sfixed64 => Self::I64(0),
            Type::Uint32 | Type::Fixed32 => Self::U32(0),
            Type::Uint64 | Type::Fixed64 => Self::U64(0),
            Type::Bool => Self::Bool(false),
            Type::String => Self::String("".to_owned()),
            Type::Bytes => Self::Bytes(vec![]),
        }
    }

    fn as_lit_expr(&self) -> proc_macro2::TokenStream {
        match *self {
            DefaultValue::F64(v) => {
                if v == f64::INFINITY {
                    quote::quote!(f64::INFINITY)
                } else if v == f64::NEG_INFINITY {
                    quote::quote!(f64::NEG_INFINITY)
                } else if v.is_nan() {
                    quote::quote!(f64::NAN)
                } else {
                    quote::quote!(#v)
                }
            }
            DefaultValue::F32(v) => {
                if v == f32::INFINITY {
                    quote::quote!(f32::INFINITY)
                } else if v == f32::NEG_INFINITY {
                    quote::quote!(f32::NEG_INFINITY)
                } else if v.is_nan() {
                    quote::quote!(f32::NAN)
                } else {
                    quote::quote!(#v)
                }
            }
            DefaultValue::I32(v) => quote::quote!(#v),
            DefaultValue::I64(v) => quote::quote!(#v),
            DefaultValue::U32(v) => quote::quote!(#v),
            DefaultValue::U64(v) => quote::quote!(#v),
            DefaultValue::Bool(v) => quote::quote!(#v),
            DefaultValue::String(ref v) => {
                let v = v.as_str();
                quote::quote!(#v)
            }
            DefaultValue::Bytes(ref v) => {
                use quote::ToTokens;
                syn::LitByteStr::new(v, proc_macro2::Span::call_site()).to_token_stream()
            }
        }
    }
}

trait FloatConsts {
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;
}

impl FloatConsts for f32 {
    const INFINITY: Self = f32::INFINITY;
    const NEG_INFINITY: Self = f32::NEG_INFINITY;
    const NAN: Self = f32::NAN;
}

impl FloatConsts for f64 {
    const INFINITY: Self = f64::INFINITY;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;
    const NAN: Self = f64::NAN;
}

fn parse_float_literal<F>(expr: &syn::Expr, type_: Type) -> Result<F, syn::Error>
where
    F: FloatConsts + str::FromStr,
    <F as str::FromStr>::Err: fmt::Display,
{
    use syn::spanned::Spanned as _;

    match expr {
        syn::Expr::Path(syn::ExprPath { path, .. }) => {
            if path.is_ident("inf") {
                Ok(<F as FloatConsts>::INFINITY)
            } else if path.is_ident("nan") {
                Ok(<F as FloatConsts>::NAN)
            } else {
                Err(syn::Error::new(
                    path.span(),
                    "Unrecognized magic word literal",
                ))
            }
        }
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Float(float),
            ..
        }) => Ok(float.base10_parse()?),
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(int),
            ..
        }) => Ok(int.base10_parse()?),
        syn::Expr::Unary(syn::ExprUnary {
            op: syn::UnOp::Neg(_),
            expr,
            ..
        }) => match **expr {
            syn::Expr::Path(syn::ExprPath { ref path, .. }) => {
                if path.is_ident("inf") {
                    Ok(<F as FloatConsts>::NEG_INFINITY)
                } else {
                    Err(syn::Error::new(
                        path.span(),
                        "Unrecognized magic word literal",
                    ))
                }
            }
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Float(ref float),
                ..
            }) => Ok(format!("-{}", float.base10_digits())
                .parse()
                .map_err(|err| syn::Error::new(expr.span(), err))?),
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(ref int),
                ..
            }) => Ok(format!("-{}", int.base10_digits())
                .parse()
                .map_err(|err| syn::Error::new(expr.span(), err))?),
            _ => Err(syn::Error::new(
                expr.span(),
                "Can't negate anything but a literal number or `inf` in this context",
            )),
        },
        _ => Err(syn::Error::new(
            expr.span(),
            format_args!(
                "Expected a float default value for a field of type `{}`",
                type_
            ),
        )),
    }
}

fn parse_int_literal<N>(expr: &syn::Expr, type_: Type) -> Result<N, syn::Error>
where
    N: str::FromStr + fmt::Display,
    <N as str::FromStr>::Err: fmt::Display,
{
    use syn::spanned::Spanned as _;

    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(int),
            ..
        }) => Ok(int.base10_parse()?),
        syn::Expr::Unary(syn::ExprUnary {
            op: syn::UnOp::Neg(_),
            expr,
            ..
        }) => match **expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(ref int),
                ..
            }) => Ok(format!("-{}", int.base10_digits())
                .parse()
                .map_err(|err| syn::Error::new(expr.span(), err))?),
            _ => Err(syn::Error::new(
                expr.span(),
                "Can't negate anything but an integer literal in this context",
            )),
        },
        _ => Err(syn::Error::new(
            expr.span(),
            format_args!(
                "Expected a literal integer default value for a field of type `{}`",
                type_
            ),
        )),
    }
}

fn parse_positive_int_literal<N>(expr: &syn::Expr, type_: Type) -> Result<N, syn::Error>
where
    N: str::FromStr + fmt::Display,
    <N as str::FromStr>::Err: fmt::Display,
{
    use syn::spanned::Spanned as _;

    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(int),
            ..
        }) => Ok(int.base10_parse()?),
        _ => Err(syn::Error::new(
            expr.span(),
            format_args!(
                "Expected a positive literal integer default value for a field of type `{}`",
                type_
            ),
        )),
    }
}
