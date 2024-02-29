use syn::spanned::Spanned;

pub mod enumeration;
pub mod message;
pub mod oneof;
pub mod scalar;
pub mod unknown_fields;

pub enum Field {
    Scalar(scalar::Field),
    Message(message::Field),
    Enumeration(enumeration::Field),
    Oneof(oneof::Field),
    UnknownFields(unknown_fields::Field),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Label {
    /// An optional field.
    Optional,
    /// A required field.
    Required,
    /// A repeated field.
    Repeated,
}

#[derive(Clone)]
pub enum Kind {
    /// A plain proto3 scalar field.
    Plain,
    /// An optional scalar field.
    Optional,
    /// A required proto2 scalar field.
    Required,
    /// A repeated scalar field.
    Repeated,
    /// A packed repeated scalar field.
    Packed,
}

#[derive(Clone, Debug, Default)]
pub struct Spec {
    pub scalar_type: Option<scalar::Type>,
    pub tag: Option<u32>,
    pub tags: Vec<u32>,
    pub default: Option<syn::Expr>,
    pub message: bool,
    pub enumeration: bool,
    pub oneof: bool,
    pub group: bool,
    pub repeated: bool,
    pub optional: bool,
    pub required: bool,
    pub packed: bool,
    // We currently don't actually use this value by pretending that ::femtopb::deferred::Deferred is a Message
    pub deferred: bool,
    pub unknown_fields: bool,
    pub phantom: bool,
    pub map: bool,
}

impl Field {
    pub fn new(
        field_span: proc_macro2::Span,
        attrs: Vec<syn::Attribute>,
    ) -> syn::Result<Option<Self>> {
        let (span, spec) = Spec::find_attr_and_parse(field_span, &attrs)?;
        Self::from_spec(span, spec)
    }

    pub fn from_spec(span: proc_macro2::Span, spec: Spec) -> syn::Result<Option<Self>> {
        if let Some(field) = scalar::Field::new(span, &spec)? {
            Ok(Some(Field::Scalar(field)))
        } else if let Some(field) = message::Field::new(span, &spec)? {
            Ok(Some(Field::Message(field)))
        } else if let Some(field) = enumeration::Field::new(span, &spec)? {
            Ok(Some(Field::Enumeration(field)))
        } else if let Some(field) = oneof::Field::new(span, &spec)? {
            Ok(Some(Field::Oneof(field)))
        } else if let Some(field) = unknown_fields::Field::new(span, &spec)? {
            Ok(Some(Field::UnknownFields(field)))
        } else if spec.phantom {
            Ok(None)
        } else if spec.group {
            Err(syn::Error::new(span, "`femtopb` currently does not support groups; you will need to remove this field from your schema for now"))
        } else if spec.map {
            Err(syn::Error::new(span, "`femtopb` currently does not support map fields; you will need to remove this field from your schema for now"))
        } else {
            Err(syn::Error::new(span, "missing type attribute on field"))
        }
    }

    pub fn tags(&self) -> impl IntoIterator<Item = u32> {
        match *self {
            Field::Scalar(ref f) => vec![f.tag],
            Field::Message(ref f) => vec![f.tag],
            Field::Enumeration(ref f) => vec![f.tag],
            Field::Oneof(ref f) => f.tags.clone(),
            Field::UnknownFields(_) => vec![],
        }
    }

    pub fn ord_key(&self) -> u32 {
        match *self {
            Field::Scalar(ref f) => f.tag,
            Field::Message(ref f) => f.tag,
            Field::Enumeration(ref f) => f.tag,
            Field::Oneof(ref f) => f.tags.iter().copied().min().unwrap_or(0),
            Field::UnknownFields(_) => u32::MAX,
        }
    }

    pub fn encode_raw_block(
        &self,
        field: &proc_macro2::TokenStream,
        cursor: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        match *self {
            Field::Scalar(ref f) => f.encode_raw_block(field, cursor),
            Field::Message(ref f) => f.encode_raw_block(field, cursor),
            Field::Enumeration(ref f) => f.encode_raw_block(field, cursor),
            Field::Oneof(ref f) => f.encode_raw_block(field, cursor),
            Field::UnknownFields(ref f) => f.encode_raw_block(field, cursor),
        }
    }

    pub fn encoded_len_expr(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<Option<proc_macro2::TokenStream>> {
        match *self {
            Field::Scalar(ref f) => f.encoded_len_expr(field).map(Some),
            Field::Message(ref f) => f.encoded_len_expr(field).map(Some),
            Field::Enumeration(ref f) => f.encoded_len_expr(field).map(Some),
            Field::Oneof(ref f) => f.encoded_len_expr(field).map(Some),
            Field::UnknownFields(_) => Ok(None),
        }
    }

    pub fn decode_match_arm(
        &self,
        matched_tag: &proc_macro2::TokenStream,
        field: &proc_macro2::TokenStream,
        wire_type: &proc_macro2::TokenStream,
        msg_buf: &proc_macro2::TokenStream,
        remaining: &proc_macro2::TokenStream,
        known_tags: &[u32],
    ) -> syn::Result<proc_macro2::TokenStream> {
        match *self {
            Field::Scalar(ref f) => f.decode_match_arm(field, wire_type, msg_buf, remaining),
            Field::Message(ref f) => f.decode_match_arm(field, wire_type, msg_buf, remaining),
            Field::Enumeration(ref f) => f.decode_match_arm(field, wire_type, msg_buf, remaining),
            Field::Oneof(ref f) => {
                f.decode_match_arm(matched_tag, field, wire_type, msg_buf, remaining)
            }
            Field::UnknownFields(ref f) => {
                f.decode_match_arm(field, wire_type, msg_buf, remaining, known_tags)
            }
        }
    }

    pub fn decode_raw_block(
        &self,
        matched_tag: &proc_macro2::TokenStream,
        field: &proc_macro2::TokenStream,
        wire_type: &proc_macro2::TokenStream,
        msg_buf: &proc_macro2::TokenStream,
        remaining: &proc_macro2::TokenStream,
        known_tags: &[u32],
    ) -> syn::Result<proc_macro2::TokenStream> {
        match *self {
            Field::Scalar(ref f) => f.decode_raw_block(field, wire_type, msg_buf, remaining),
            Field::Message(ref f) => f.decode_raw_block(field, wire_type, msg_buf, remaining),
            Field::Enumeration(ref f) => f.decode_raw_block(field, wire_type, msg_buf, remaining),
            Field::Oneof(ref f) => {
                f.decode_raw_block(matched_tag, field, wire_type, msg_buf, remaining)
            }
            Field::UnknownFields(ref f) => {
                f.decode_raw_block(field, wire_type, msg_buf, remaining, known_tags)
            }
        }
    }

    pub fn clear_block(
        &self,
        field: &proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        match *self {
            Field::Scalar(ref f) => f.clear_block(field),
            Field::Message(ref f) => f.clear_block(field),
            Field::Enumeration(ref f) => f.clear_block(field),
            Field::Oneof(ref f) => f.clear_block(field),
            Field::UnknownFields(ref f) => f.clear_block(field),
        }
    }

    pub fn default_expr(&self) -> syn::Result<proc_macro2::TokenStream> {
        match *self {
            Field::Scalar(ref f) => f.default_expr(),
            Field::Message(ref f) => f.default_expr(),
            Field::Enumeration(ref f) => f.default_expr(),
            Field::Oneof(ref f) => f.default_expr(),
            Field::UnknownFields(ref f) => f.default_expr(),
        }
    }
}

impl Spec {
    pub fn find_attr_and_parse(
        field_span: proc_macro2::Span,
        attrs: &[syn::Attribute],
    ) -> syn::Result<(proc_macro2::Span, Self)> {
        if let Some(attr) = attrs.into_iter().find(|a| a.path().is_ident("femtopb")) {
            let spec = Self::parse(&attr)?;
            Ok((attr.span(), spec))
        } else {
            Err(syn::Error::new(
                field_span,
                "field lacks a `femtopb` annotation",
            ))
        }
    }

    pub fn parse(raw: &syn::Attribute) -> syn::Result<Self> {
        use syn::spanned::Spanned as _;

        assert!(raw.path().is_ident("femtopb"));

        let mut spec = Spec::default();
        raw.parse_nested_meta(|meta| {
            let path = &meta.path;
            if path.is_ident("int32") {
                spec.set_scalar_type(path.span(), scalar::Type::Int32)?;
                Ok(())
            } else if path.is_ident("int64") {
                spec.set_scalar_type(path.span(), scalar::Type::Int64)?;
                Ok(())
            } else if path.is_ident("uint32") {
                spec.set_scalar_type(path.span(), scalar::Type::Uint32)?;
                Ok(())
            } else if path.is_ident("uint64") {
                spec.set_scalar_type(path.span(), scalar::Type::Uint64)?;
                Ok(())
            } else if path.is_ident("sint32") {
                spec.set_scalar_type(path.span(), scalar::Type::Sint32)?;
                Ok(())
            } else if path.is_ident("sint64") {
                spec.set_scalar_type(path.span(), scalar::Type::Sint64)?;
                Ok(())
            } else if path.is_ident("fixed32") {
                spec.set_scalar_type(path.span(), scalar::Type::Fixed32)?;
                Ok(())
            } else if path.is_ident("fixed64") {
                spec.set_scalar_type(path.span(), scalar::Type::Fixed64)?;
                Ok(())
            } else if path.is_ident("sfixed32") {
                spec.set_scalar_type(path.span(), scalar::Type::Sfixed32)?;
                Ok(())
            } else if path.is_ident("sfixed64") {
                spec.set_scalar_type(path.span(), scalar::Type::Sfixed64)?;
                Ok(())
            } else if path.is_ident("float") {
                spec.set_scalar_type(path.span(), scalar::Type::Float)?;
                Ok(())
            } else if path.is_ident("double") {
                spec.set_scalar_type(path.span(), scalar::Type::Double)?;
                Ok(())
            } else if path.is_ident("bool") {
                spec.set_scalar_type(path.span(), scalar::Type::Bool)?;
                Ok(())
            } else if path.is_ident("string") {
                spec.set_scalar_type(path.span(), scalar::Type::String)?;
                Ok(())
            } else if path.is_ident("bytes") {
                spec.set_scalar_type(path.span(), scalar::Type::Bytes)?;
                Ok(())
            } else if path.is_ident("message") {
                spec.message = true;
                Ok(())
            } else if path.is_ident("enumeration") {
                spec.enumeration = true;
                Ok(())
            } else if path.is_ident("oneof") {
                spec.oneof = true;
                Ok(())
            } else if path.is_ident("group") {
                spec.group = true;
                Ok(())
            } else if path.is_ident("repeated") {
                spec.repeated = true;
                Ok(())
            } else if path.is_ident("optional") {
                spec.optional = true;
                Ok(())
            } else if path.is_ident("required") {
                spec.required = true;
                Ok(())
            } else if path.is_ident("packed") {
                spec.packed = true;
                Ok(())
            } else if path.is_ident("deferred") {
                spec.deferred = true;
                Ok(())
            } else if path.is_ident("unknown_fields") {
                spec.unknown_fields = true;
                Ok(())
            } else if path.is_ident("phantom") {
                spec.phantom = true;
                Ok(())
            } else if path.is_ident("map")
                || path.is_ident("btree_map")
                || path.is_ident("hash_map")
            {
                spec.map = true;
                Ok(())
            } else if path.is_ident("tag") {
                meta.input.parse::<syn::Token![=]>()?;
                let int = meta.input.parse::<syn::LitInt>()?.base10_parse()?;
                spec.set_tag(path.span(), int)?;
                Ok(())
            } else if path.is_ident("tags") {
                use syn::parse::Parse as _;
                meta.input.parse::<syn::Token![=]>()?;
                let content;
                syn::bracketed!(content in meta.input);
                let tags = content.parse_terminated(syn::LitInt::parse, syn::Token![,])?;
                let tags = tags
                    .into_iter()
                    .map(|i| Ok(i.base10_parse()?))
                    .collect::<syn::Result<Vec<_>>>()?;
                spec.set_tags(path.span(), tags)?;
                Ok(())
            } else if path.is_ident("default") {
                meta.input.parse::<syn::Token![=]>()?;
                let expr = meta.input.parse::<syn::Expr>()?;
                spec.set_default(path.span(), expr)?;
                Ok(())
            } else {
                Err(syn::Error::new(
                    path.span(),
                    "unrecognized femtopb attribute argument",
                ))
            }
        })?;
        Ok(spec)
    }

    fn set_scalar_type(&mut self, span: proc_macro2::Span, ty: scalar::Type) -> syn::Result<()> {
        if let Some(ty) = self.scalar_type.replace(ty) {
            Err(syn::Error::new(
                span,
                format_args!("Duplicate type attribute; already saw type `{}`", ty),
            ))
        } else {
            Ok(())
        }
    }

    fn set_tag(&mut self, span: proc_macro2::Span, tag: u32) -> syn::Result<()> {
        if !self.tags.is_empty() {
            Err(syn::Error::new(span, "Can't specify both `tags` and `tag`"))
        } else if let Some(tag) = self.tag.replace(tag) {
            Err(syn::Error::new(
                span,
                format_args!("Duplicate tag spec; already saw tag `{}`", tag),
            ))
        } else {
            Ok(())
        }
    }

    fn set_tags(&mut self, span: proc_macro2::Span, tags: Vec<u32>) -> syn::Result<()> {
        if self.tag.is_some() {
            Err(syn::Error::new(span, "Can't specify both `tag` and `tags`"))
        } else if tags.is_empty() {
            Err(syn::Error::new(
                span,
                "Must specify at least one tag in the `tags` array",
            ))
        } else {
            self.tags = tags;
            Ok(())
        }
    }

    fn set_default(&mut self, span: proc_macro2::Span, default: syn::Expr) -> syn::Result<()> {
        use quote::ToTokens as _;

        if let Some(prev) = self.default.replace(default) {
            Err(syn::Error::new(
                span,
                format_args!(
                    "Duplicate default value spec; already saw default value `{}`",
                    prev.to_token_stream()
                ),
            ))
        } else {
            Ok(())
        }
    }

    fn ensure_not_packed(
        &self,
        span: proc_macro2::Span,
        conflicting_attr: &str,
    ) -> syn::Result<()> {
        if self.packed {
            Err(syn::Error::new(
                span,
                format_args!("field cannot be both `{}` and `packed`", conflicting_attr),
            ))
        } else {
            Ok(())
        }
    }

    fn ensure_not_repeated(
        &self,
        span: proc_macro2::Span,
        conflicting_attr: &str,
    ) -> syn::Result<()> {
        if self.repeated {
            Err(syn::Error::new(
                span,
                format_args!("field cannot be both `{}` and `repeated`", conflicting_attr),
            ))
        } else {
            Ok(())
        }
    }

    fn ensure_not_required(
        &self,
        span: proc_macro2::Span,
        conflicting_attr: &str,
    ) -> syn::Result<()> {
        if self.required {
            Err(syn::Error::new(
                span,
                format_args!("field cannot be both `{}` and `required`", conflicting_attr),
            ))
        } else {
            Ok(())
        }
    }

    fn ensure_not_optional(
        &self,
        span: proc_macro2::Span,
        conflicting_attr: &str,
    ) -> syn::Result<()> {
        if self.optional {
            Err(syn::Error::new(
                span,
                format_args!("field cannot be both `{}` and `optional`", conflicting_attr),
            ))
        } else {
            Ok(())
        }
    }
}

impl Label {
    pub fn from_spec(span: proc_macro2::Span, spec: &Spec) -> syn::Result<Option<Self>> {
        if spec.optional {
            spec.ensure_not_required(span, "optional")?;
            spec.ensure_not_repeated(span, "optional")?;
            Ok(Some(Label::Optional))
        } else if spec.required {
            spec.ensure_not_optional(span, "required")?;
            spec.ensure_not_repeated(span, "required")?;
            Ok(Some(Label::Required))
        } else if spec.repeated {
            spec.ensure_not_optional(span, "repeated")?;
            spec.ensure_not_required(span, "repeated")?;
            Ok(Some(Label::Repeated))
        } else if spec.packed {
            Err(syn::Error::new(
                span,
                format_args!("a field with a message type cannot be `packed`"),
            ))
        } else {
            Ok(None)
        }
    }
}

impl Kind {
    pub fn from_spec(span: proc_macro2::Span, spec: &Spec) -> syn::Result<Self> {
        if spec.packed {
            spec.ensure_not_repeated(span, "packed")?;
            spec.ensure_not_required(span, "packed")?;
            spec.ensure_not_optional(span, "packed")?;
            Ok(Kind::Packed)
        } else if spec.repeated {
            spec.ensure_not_packed(span, "repeated")?;
            spec.ensure_not_required(span, "repeated")?;
            spec.ensure_not_optional(span, "repeated")?;
            Ok(Kind::Repeated)
        } else if spec.required {
            spec.ensure_not_packed(span, "required")?;
            spec.ensure_not_repeated(span, "required")?;
            spec.ensure_not_optional(span, "required")?;
            Ok(Kind::Required)
        } else if spec.optional {
            spec.ensure_not_packed(span, "optional")?;
            spec.ensure_not_repeated(span, "optional")?;
            spec.ensure_not_required(span, "optional")?;
            Ok(Kind::Optional)
        } else {
            Ok(Kind::Plain)
        }
    }
}
