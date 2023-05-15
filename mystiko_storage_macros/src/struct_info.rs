use crate::field_info::FieldInfo;
use heck::ToUpperCamelCase;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;

pub struct StructInfo<'a> {
    pub name: &'a syn::Ident,
    pub fields: Vec<FieldInfo<'a>>,
    pub attributes: StructAttribute,
}

pub struct StructAttribute {
    pub collection_name: Option<syn::Expr>,
    pub unique_columns: Option<syn::Expr>,
    pub migrations: Option<syn::Expr>,
}

impl<'a> StructInfo<'a> {
    pub fn new(
        ast: &'a syn::DeriveInput,
        fields: impl Iterator<Item = &'a syn::Field>,
    ) -> Result<StructInfo<'a>, syn::parse::Error> {
        Ok(StructInfo {
            name: &ast.ident,
            fields: fields
                .enumerate()
                .map(|(i, f)| FieldInfo::new(i, f))
                .collect::<Result<_, _>>()?,
            attributes: StructAttribute::new(&ast.attrs)?,
        })
    }

    pub fn column_enum_impl(&self) -> Result<TokenStream, syn::parse::Error> {
        let enum_name = syn::Ident::new(&format!("{}Column", self.name), self.name.span());
        let enum_item_names = self
            .fields
            .iter()
            .map(|f| {
                let ident = f.name;
                let name = ident.to_string().to_upper_camel_case();
                syn::Ident::new(&name, ident.span())
            })
            .collect::<Vec<_>>();
        let to_string_values = self.fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        let enum_items1 = enum_item_names.iter();
        let enum_items2 = enum_item_names.iter();
        let to_string_items = to_string_values.iter();
        Ok(quote! {
            pub enum #enum_name {
                #(#enum_items1,)*
            }
            impl ToString for #enum_name {
                fn to_string(&self) -> String {
                    match self {
                        #( Self::#enum_items2 => stringify!(#to_string_items).to_string(), )*
                    }
                }
            }
        })
    }

    pub fn document_data_impl(&self) -> Result<TokenStream, syn::parse::Error> {
        let struct_name = self.name.clone();
        let collection_name = syn::Ident::new(&self.name.to_string().to_snake_case().to_plural(), self.name.span());
        let collection_name = if let Some(collection_name_override) = self.attributes.collection_name.as_ref() {
            quote! { #collection_name_override }
        } else {
            quote! { stringify!(#collection_name) }
        };
        let column_creates = self.fields.iter().map(|f| f.create_impl()).collect::<Vec<_>>();
        let column_creates_iter = column_creates.iter();
        let columns = self.fields.iter().map(|f| f.column_impl()).collect::<Vec<_>>();
        let column_iter = columns.iter();
        let column_values = self.fields.iter().map(|f| f.column_value_impl()).collect::<Vec<_>>();
        let column_values_iter = column_values.iter();
        let unique_columns = if let Some(unique_columns) = self.attributes.unique_columns.as_ref() {
            quote! { #unique_columns }
        } else {
            quote! { vec![vec![]] }
        };
        let migrations = if let Some(migrations) = self.attributes.migrations.as_ref() {
            quote! { #migrations }
        } else {
            quote! { vec![] }
        };
        Ok(quote! {
            impl mystiko_storage2::document::DocumentData for #struct_name {
                fn create(
                    column_values: &[(&str, mystiko_storage2::column::ColumnValue)]
                ) -> anyhow::Result<Self, mystiko_storage2::error::StorageError> {
                    Ok(Self { #(#column_creates_iter,)* })
                }
                fn collection_name() -> String {
                    #collection_name.to_string()
                }
                fn columns() -> Vec<mystiko_storage2::column::Column> {
                   vec![#(#column_iter,)*]
                }
                fn column_values(&self) ->
                    Vec<(mystiko_storage2::column::Column, Option<mystiko_storage2::column::ColumnValue>)> {
                        Self::columns()
                            .into_iter()
                            .zip(vec![#(#column_values_iter,)*])
                            .collect()
                }
                fn unique_columns() -> Vec<Vec<String>> {
                    #unique_columns
                }
                fn migrations() -> Vec<mystiko_storage2::migration::types::Migration> {
                    #migrations
                }
            }
        })
    }

    pub fn collection_impl(&self) -> Result<TokenStream, syn::parse::Error> {
        let struct_name = self.name.clone();
        let collection_struct_name = syn::Ident::new(&format!("{}Collection", self.name), self.name.span());
        Ok(quote! {
            #[derive(Debug)]
            pub struct #collection_struct_name<
                F: mystiko_storage2::formatter::types::StatementFormatter, S: mystiko_storage2::storage::Storage> {
                collection: std::sync::Arc<mystiko_storage2::collection::Collection<F, S>>,
            }
            impl <F, S> #collection_struct_name<F, S>
            where
                F: mystiko_storage2::formatter::types::StatementFormatter,
                S: mystiko_storage2::storage::Storage,
            {
                pub fn new(collection: std::sync::Arc<mystiko_storage2::collection::Collection<F, S>>) -> Self {
                    #collection_struct_name { collection }
                }

                pub async fn insert(&self, document: &#struct_name) -> anyhow::Result<
                        mystiko_storage2::document::Document<#struct_name>,
                        mystiko_storage2::error::StorageError> {
                    self.collection.insert(document).await
                }
                pub async fn insert_batch(&self, documents: &Vec<#struct_name>) -> anyhow::Result<
                        Vec<mystiko_storage2::document::Document<#struct_name>>,
                        mystiko_storage2::error::StorageError> {
                    self.collection.insert_batch(documents).await
                }
                pub async fn find<Q: Into<mystiko_storage2::filter::QueryFilter>>(
                    &self, filter: Q) -> anyhow::Result<
                        Vec<mystiko_storage2::document::Document<#struct_name>>,
                        mystiko_storage2::error::StorageError> {
                    self.collection.find(Some(filter)).await
                }
                pub async fn find_all<Q: Into<mystiko_storage2::filter::QueryFilter>>(&self) -> anyhow::Result<
                        Vec<mystiko_storage2::document::Document<#struct_name>>,
                        mystiko_storage2::error::StorageError> {
                    self.collection.find::<#struct_name, Q>(None).await
                }
                pub async fn find_one<Q: Into<mystiko_storage2::filter::QueryFilter>>(
                    &self, filter: Q) -> anyhow::Result<
                        Option<mystiko_storage2::document::Document<#struct_name>>,
                        mystiko_storage2::error::StorageError> {
                    self.collection.find_one(Some(filter)).await
                }
                pub async fn find_by_id(&self, id: &str) -> anyhow::Result<
                        Option<mystiko_storage2::document::Document<#struct_name>>,
                        mystiko_storage2::error::StorageError> {
                    self.collection.find_by_id(id).await
                }
                pub async fn count<Q: Into<mystiko_storage2::filter::QueryFilter>>(
                    &self, filter: Q) -> anyhow::Result<u64, mystiko_storage2::error::StorageError> {
                    self.collection.count::<#struct_name, Q>(Some(filter)).await
                }
                pub async fn count_all<Q: Into<mystiko_storage2::filter::QueryFilter>>(&self) -> anyhow::Result<
                        u64, mystiko_storage2::error::StorageError> {
                    self.collection.count::<#struct_name, Q>(None).await
                }
                pub async fn update(&self, document: &mystiko_storage2::document::Document<#struct_name>) ->
                    anyhow::Result<
                        mystiko_storage2::document::Document<#struct_name>,
                        mystiko_storage2::error::StorageError> {
                    self.collection.update(document).await
                }
                pub async fn update_batch(&self,
                    documents: &Vec<mystiko_storage2::document::Document<#struct_name>>) ->
                        anyhow::Result<
                            Vec<mystiko_storage2::document::Document<#struct_name>>,
                            mystiko_storage2::error::StorageError> {
                    self.collection.update_batch(documents).await
                }
                pub async fn delete(&self, document: &mystiko_storage2::document::Document<#struct_name>) ->
                    anyhow::Result<(), mystiko_storage2::error::StorageError> {
                    self.collection.delete(document).await
                }
                pub async fn delete_batch(&self,
                    documents: &Vec<mystiko_storage2::document::Document<#struct_name>>) ->
                        anyhow::Result<(), mystiko_storage2::error::StorageError> {
                    self.collection.delete_batch(documents).await
                }
                pub async fn delete_all(&self) -> anyhow::Result<(), mystiko_storage2::error::StorageError> {
                    self.collection.delete_by_filter::<#struct_name, mystiko_storage2::filter::QueryFilter>(None).await
                }
                pub async fn delete_by_filter<Q: Into<mystiko_storage2::filter::QueryFilter>>(&self, filter: Q) ->
                    anyhow::Result<(), mystiko_storage2::error::StorageError> {
                    self.collection.delete_by_filter::<#struct_name,Q>(Some(filter)).await
                }
                pub async fn collection_exists(&self) ->
                    anyhow::Result<bool, mystiko_storage2::error::StorageError> {
                    self.collection.collection_exists(&#struct_name::collection_name()).await
                }
                pub async fn migrate(&self) -> Result<
                        mystiko_storage2::document::Document<mystiko_storage2::migration::history::MigrationHistory>,
                        mystiko_storage2::error::StorageError> {
                    self.collection.migrate::<#struct_name>().await
                }
            }
        })
    }
}

impl StructAttribute {
    pub fn new(attrs: &[syn::Attribute]) -> Result<Self, syn::parse::Error> {
        let mut collection_name: Option<syn::Expr> = None;
        let mut unique_columns: Option<syn::Expr> = None;
        let mut migrations: Option<syn::Expr> = None;
        for attr in attrs {
            if attr.path().is_ident("collection") {
                attr.parse_nested_meta(|meta| {
                    let expr: syn::Expr = meta.value()?.parse()?;
                    if meta.path.is_ident("name") {
                        collection_name = Some(expr);
                    } else if meta.path.is_ident("uniques") {
                        unique_columns = Some(expr);
                    } else if meta.path.is_ident("migrations") {
                        migrations = Some(expr);
                    }
                    Ok(())
                })?;
            }
        }
        Ok(Self {
            collection_name,
            unique_columns,
            migrations,
        })
    }
}
