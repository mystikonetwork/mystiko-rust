use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::Error;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ProtoBuilder)]
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
            syn::Fields::Named(_) => {
                let try_from_u8_array_ref = impl_try_from_u8_array_ref(ast.ident.clone());
                let try_from_u8_vec = impl_try_from_u8_vec(ast.ident.clone());
                let try_from_u8_vec_ref = impl_try_from_u8_vec_ref(ast.ident.clone());
                quote! {
                    #try_from_u8_array_ref
                    #try_from_u8_vec
                    #try_from_u8_vec_ref
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
        syn::Data::Enum(_) => return Err(Error::new(ast.span(), "ProtoBuilder is not supported for enums")),
        syn::Data::Union(_) => return Err(Error::new(ast.span(), "ProtoBuilder is not supported for unions")),
    };
    Ok(data)
}

fn impl_try_from_u8_array_ref(struct_name: Ident) -> TokenStream {
    quote! {
        impl TryFrom<&[u8]> for #struct_name {
            type Error = prost::DecodeError;
            fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                prost::Message::decode(bytes)
            }
        }
    }
}

fn impl_try_from_u8_vec(struct_name: Ident) -> TokenStream {
    quote! {
        impl TryFrom<Vec<u8>> for #struct_name {
            type Error = prost::DecodeError;
            fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                prost::Message::decode(bytes.as_slice())
            }
        }
    }
}

fn impl_try_from_u8_vec_ref(struct_name: Ident) -> TokenStream {
    quote! {
        impl TryFrom<&Vec<u8>> for #struct_name {
            type Error = prost::DecodeError;
            fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
                prost::Message::decode(bytes.as_slice())
            }
        }
    }
}
