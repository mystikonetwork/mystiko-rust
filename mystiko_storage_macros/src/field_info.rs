use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub struct FieldInfo<'a> {
    pub name: &'a syn::Ident,
    pub column_enum_name: syn::Ident,
    pub column_enum_item_name: syn::Ident,
    pub type_name: String,
    pub type_name_enum: String,
    pub is_optional: bool,
    pub is_primitive: bool,
    pub attributes: FieldAttribute,
}

pub struct FieldAttribute {
    pub length_limit: Option<syn::Expr>,
}

impl<'a> FieldInfo<'a> {
    pub fn new(
        field: &syn::Field,
        column_enum_name: syn::Ident,
        column_enum_item_name: syn::Ident,
    ) -> Result<FieldInfo, syn::parse::Error> {
        if let Some(ref name) = field.ident {
            let ty = option_inside_type(&field.ty);
            let type_name = type_name(ty)?;
            Ok(FieldInfo {
                name,
                column_enum_name,
                column_enum_item_name,
                type_name_enum: type_name_enum(&type_name)?,
                is_optional: is_optional_type(&field.ty),
                is_primitive: is_primitive_type(&type_name)?,
                type_name,
                attributes: FieldAttribute::new(&field.attrs)?,
            })
        } else {
            Err(syn::Error::new(field.span(), "Nameless field in struct"))
        }
    }

    pub fn create_impl(&self) -> TokenStream {
        let as_type = if self.type_name == "json" {
            syn::Ident::new("as_json_object", self.name.span())
        } else {
            syn::Ident::new(&format!("as_{}", self.type_name.to_lowercase()), self.name.span())
        };
        let field_name = self.name.clone();
        let column_enum_name = self.column_enum_name.clone();
        let column_enum_item_name = self.column_enum_item_name.clone();
        let column_finder = syn::Ident::new(
            if self.is_optional {
                "find_column_value"
            } else {
                "find_required_column_value"
            },
            self.name.span(),
        );
        if self.is_optional {
            quote! {
                #field_name: match mystiko_storage::#column_finder(
                    &#column_enum_name::#column_enum_item_name, column_values) {
                    Some(value) => Some(value.#as_type()?),
                    None => None,
                }
            }
        } else {
            quote! {
                #field_name: mystiko_storage::#column_finder(
                    &#column_enum_name::#column_enum_item_name, column_values)?.#as_type()?
            }
        }
    }

    pub fn column_impl(&self) -> TokenStream {
        let column_type = syn::Ident::new(&self.type_name_enum, self.name.span());
        let column_enum_name = self.column_enum_name.clone();
        let column_enum_item_name = self.column_enum_item_name.clone();
        let nullable = syn::Ident::new(if self.is_optional { "true" } else { "false" }, self.name.span());
        let length_limit = match self.attributes.length_limit.as_ref() {
            Some(expr) => quote! { Some(#expr) },
            None => quote! { None },
        };
        quote! {
            mystiko_storage::Column::builder()
                .column_name(#column_enum_name::#column_enum_item_name)
                .column_type(mystiko_protos::storage::v1::ColumnType::#column_type)
                .nullable(#nullable)
                .length_limit(#length_limit)
                .build()
        }
    }

    pub fn column_value_impl(&self) -> TokenStream {
        let field_name = self.name.clone();
        if self.is_optional {
            if self.type_name == "json" {
                quote! {
                    match self.#field_name.as_ref() {
                        Some(v) => serde_json::to_value(v).ok().map(|v| v.into()),
                        None => None,
                    }
                }
            } else if self.is_primitive {
                quote! {
                    self.#field_name.map(|v| v.into())
                }
            } else {
                quote! {
                    self.#field_name.as_ref().map(|v| v.clone().into())
                }
            }
        } else if self.type_name == "json" {
            quote! {
                serde_json::to_value(&self.#field_name).ok().map(|v| v.into())
            }
        } else if self.is_primitive {
            quote! {
                Some(self.#field_name.into())
            }
        } else {
            quote! {
                Some(self.#field_name.clone().into())
            }
        }
    }
}

impl FieldAttribute {
    pub fn new(attrs: &[syn::Attribute]) -> Result<Self, syn::parse::Error> {
        let mut length_limit: Option<syn::Expr> = None;
        for attr in attrs {
            if attr.path().is_ident("column") {
                attr.parse_nested_meta(|meta| {
                    let expr: syn::Expr = meta.value()?.parse()?;
                    if meta.path.is_ident("length_limit") {
                        length_limit = Some(expr);
                    }
                    Ok(())
                })?;
            }
        }
        Ok(Self { length_limit })
    }
}

fn is_optional_type(ty: &syn::Type) -> bool {
    let path = if let syn::Type::Path(type_path) = ty {
        if type_path.qself.is_some() {
            return false;
        }
        &type_path.path
    } else {
        return false;
    };
    match path.segments.last() {
        Some(segment) => segment.ident == "Option",
        None => false,
    }
}

fn type_name(ty: &syn::Type) -> Result<String, syn::parse::Error> {
    if let syn::Type::Path(type_path) = ty {
        if type_path.qself.is_some() {
            return Err(syn::Error::new(ty.span(), "Qualified self in type path"));
        }
        if let Some(segment) = type_path.path.segments.last() {
            return match segment.ident.to_string().as_str() {
                "bool" => Ok("bool".to_string()),
                "char" => Ok("char".to_string()),
                "u8" => Ok("u8".to_string()),
                "u16" => Ok("u16".to_string()),
                "u32" => Ok("u32".to_string()),
                "u64" => Ok("u64".to_string()),
                "u128" => Ok("u128".to_string()),
                "usize" => Ok("usize".to_string()),
                "i8" => Ok("i8".to_string()),
                "i16" => Ok("i16".to_string()),
                "i32" => Ok("i32".to_string()),
                "i64" => Ok("i64".to_string()),
                "i128" => Ok("i128".to_string()),
                "isize" => Ok("isize".to_string()),
                "f32" => Ok("f32".to_string()),
                "f64" => Ok("f64".to_string()),
                "String" => Ok("String".to_string()),
                "BigInt" => Ok("BigInt".to_string()),
                "BigUint" => Ok("BigUint".to_string()),
                _ => Ok("json".to_string()),
            };
        }
    }
    Err(syn::Error::new(ty.span(), "Not a named type"))
}

fn type_name_enum(type_name: &str) -> Result<String, syn::parse::Error> {
    match type_name {
        "bool" => Ok("Bool".to_string()),
        "char" => Ok("Char".to_string()),
        "u8" => Ok("U8".to_string()),
        "u16" => Ok("U16".to_string()),
        "u32" => Ok("U32".to_string()),
        "u64" => Ok("U64".to_string()),
        "u128" => Ok("U128".to_string()),
        "usize" => Ok("USize".to_string()),
        "i8" => Ok("I8".to_string()),
        "i16" => Ok("I16".to_string()),
        "i32" => Ok("I32".to_string()),
        "i64" => Ok("I64".to_string()),
        "i128" => Ok("I128".to_string()),
        "isize" => Ok("ISize".to_string()),
        "f32" => Ok("F32".to_string()),
        "f64" => Ok("F64".to_string()),
        "String" => Ok("String".to_string()),
        "BigInt" => Ok("BigInt".to_string()),
        "BigUint" => Ok("BigUint".to_string()),
        _ => Ok("Json".to_string()),
    }
}

fn is_primitive_type(type_name: &str) -> Result<bool, syn::parse::Error> {
    match type_name {
        "String" => Ok(false),
        "BigInt" => Ok(false),
        "BigUint" => Ok(false),
        "json" => Ok(false),
        _ => Ok(true),
    }
}

fn option_inside_type(ty: &syn::Type) -> &syn::Type {
    if let syn::Type::Path(type_path) = ty {
        if type_path.qself.is_none() {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Option" {
                    if let syn::PathArguments::AngleBracketed(generic_params) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inside_type)) = generic_params.args.first() {
                            return inside_type;
                        }
                    }
                }
            }
        }
    };
    ty
}
