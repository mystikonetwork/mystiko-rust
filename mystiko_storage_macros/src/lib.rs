use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Error;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};

mod field_info;
mod struct_info;

#[proc_macro_derive(CollectionBuilder, attributes(collection, column))]
pub fn derive_collection_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_my_derive(&input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn impl_my_derive(ast: &DeriveInput) -> Result<TokenStream, Error> {
    let data = match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => {
                let struct_info = struct_info::StructInfo::new(ast, fields.named.iter())?;
                let column_enum = struct_info.column_enum_impl()?;
                let document_data_impl = struct_info.document_data_impl()?;
                let collection_impl = struct_info.collection_impl()?;
                quote! {
                    #column_enum
                    #document_data_impl
                    #collection_impl
                }
            }
            syn::Fields::Unnamed(_) => {
                return Err(Error::new(
                    ast.span(),
                    "CollectionBuilder is not supported for tuple structs",
                ))
            }
            syn::Fields::Unit => {
                return Err(Error::new(
                    ast.span(),
                    "CollectionBuilder is not supported for unit structs",
                ))
            }
        },
        syn::Data::Enum(_) => return Err(Error::new(ast.span(), "CollectionBuilder is not supported for enums")),
        syn::Data::Union(_) => return Err(Error::new(ast.span(), "CollectionBuilder is not supported for unions")),
    };
    Ok(data)
}
