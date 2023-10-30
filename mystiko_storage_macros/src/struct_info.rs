use crate::field_info::FieldInfo;
use heck::ToUpperCamelCase;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub struct StructInfo<'a> {
    pub name: &'a syn::Ident,
    pub column_enum_name: syn::Ident,
    pub fields: Vec<FieldInfo<'a>>,
    pub attributes: StructAttribute,
}

pub struct StructAttribute {
    pub collection_name: Option<syn::Expr>,
    pub unique_columns: Option<syn::Expr>,
    pub index_columns: Option<syn::Expr>,
    pub migrations: Option<syn::Expr>,
}

impl<'a> StructInfo<'a> {
    pub fn new(
        ast: &'a syn::DeriveInput,
        fields: impl Iterator<Item = &'a syn::Field>,
    ) -> Result<StructInfo<'a>, syn::parse::Error> {
        let column_enum_name = syn::Ident::new(&format!("{}Column", ast.ident), ast.ident.span());
        Ok(StructInfo {
            name: &ast.ident,
            column_enum_name: column_enum_name.clone(),
            fields: fields
                .enumerate()
                .map(|(_, f)| {
                    let column_enum_item_name = syn::Ident::new(
                        &f.ident
                            .as_ref()
                            .map(|name| format!("{}", name).to_upper_camel_case())
                            .unwrap_or("".into()),
                        f.ident.span(),
                    );
                    FieldInfo::new(f, column_enum_name.clone(), column_enum_item_name)
                })
                .collect::<Result<_, _>>()?,
            attributes: StructAttribute::new(&ast.attrs)?,
        })
    }

    pub fn column_enum_impl(&self) -> Result<TokenStream, syn::parse::Error> {
        let enum_name = self.column_enum_name.clone();
        let enum_item_names = self
            .fields
            .iter()
            .map(|f| f.column_enum_item_name.clone())
            .collect::<Vec<_>>();
        let to_string_values = self.fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        let enum_items1 = enum_item_names.iter();
        let enum_items2 = enum_item_names.iter();
        let to_string_items = to_string_values.iter();
        Ok(quote! {
            pub enum #enum_name {
                #(#enum_items1,)*
            }
            impl AsRef<str> for #enum_name {
                fn as_ref(&self) -> &str {
                    match self {
                        #( Self::#enum_items2 => stringify!(#to_string_items), )*
                    }
                }
            }
            impl ToString for #enum_name {
                fn to_string(&self) -> String {
                    self.as_ref().to_string()
                }
            }
            impl From<#enum_name> for String {
                fn from(value: #enum_name) -> Self {
                    value.to_string()
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
            quote! { vec![] }
        };
        let index_columns = if let Some(index_columns) = self.attributes.index_columns.as_ref() {
            quote! { #index_columns }
        } else {
            quote! { vec![] }
        };
        let migrations = if let Some(migrations) = self.attributes.migrations.as_ref() {
            quote! { #migrations }
        } else {
            quote! { vec![] }
        };
        Ok(quote! {
            impl mystiko_storage::DocumentData for #struct_name {
                fn create(
                    column_values: &[(String, mystiko_protos::storage::v1::ColumnValue)]
                ) -> anyhow::Result<Self, mystiko_storage::StorageError> {
                    Ok(Self { #(#column_creates_iter,)* })
                }
                fn collection_name() -> &'static str {
                    #collection_name
                }
                fn columns() -> Vec<mystiko_storage::Column> {
                   vec![#(#column_iter,)*]
                }
                fn column_values(&self) ->
                    Vec<(mystiko_storage::Column, Option<mystiko_protos::storage::v1::ColumnValue>)> {
                        Self::columns()
                            .into_iter()
                            .zip(vec![#(#column_values_iter,)*])
                            .collect()
                }
                fn unique_columns() -> Vec<mystiko_storage::UniqueColumns> {
                    #unique_columns
                }
                fn index_columns() -> Vec<mystiko_storage::IndexColumns> {
                    #index_columns
                }
                fn migrations() -> Vec<mystiko_storage::Migration> {
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
                F: mystiko_storage::StatementFormatter, S: mystiko_storage::Storage> {
                collection: std::sync::Arc<mystiko_storage::Collection<F, S>>,
            }
            impl <F, S> #collection_struct_name<F, S>
            where
                F: mystiko_storage::StatementFormatter,
                S: mystiko_storage::Storage,
            {
                pub fn new(collection: std::sync::Arc<mystiko_storage::Collection<F, S>>) -> Self {
                    #collection_struct_name { collection }
                }

                pub async fn insert(&self, document: &#struct_name) -> anyhow::Result<
                        mystiko_storage::Document<#struct_name>,
                        mystiko_storage::StorageError> {
                    self.collection.insert(document).await
                }
                pub async fn insert_batch(&self, documents: &[#struct_name]) -> anyhow::Result<
                        Vec<mystiko_storage::Document<#struct_name>>,
                        mystiko_storage::StorageError> {
                    self.collection.insert_batch(documents).await
                }
                pub async fn find<Q: Into<mystiko_protos::storage::v1::QueryFilter>>(
                    &self, filter: Q) -> anyhow::Result<
                        Vec<mystiko_storage::Document<#struct_name>>,
                        mystiko_storage::StorageError> {
                    self.collection.find(Some(filter)).await
                }
                pub async fn find_all(&self) -> anyhow::Result<
                        Vec<mystiko_storage::Document<#struct_name>>,
                        mystiko_storage::StorageError> {
                    self.collection.find::<#struct_name, mystiko_protos::storage::v1::QueryFilter>(None).await
                }
                pub async fn find_one<Q>(
                    &self, filter: Q) -> anyhow::Result<
                        Option<mystiko_storage::Document<#struct_name>>,
                        mystiko_storage::StorageError>
                where
                    Q: Into<mystiko_protos::storage::v1::QueryFilter>
                {
                    self.collection.find_one(Some(filter)).await
                }
                pub async fn find_by_id(&self, id: &str) -> anyhow::Result<
                        Option<mystiko_storage::Document<#struct_name>>,
                        mystiko_storage::StorageError> {
                    self.collection.find_by_id(id).await
                }
                pub async fn count<Q>(
                    &self, filter: Q) -> anyhow::Result<u64, mystiko_storage::StorageError>
                where
                    Q: Into<mystiko_protos::storage::v1::QueryFilter>
                {
                    self.collection.count::<#struct_name, Q>(Some(filter)).await
                }
                pub async fn count_all(&self) -> anyhow::Result<
                        u64, mystiko_storage::StorageError> {
                    self.collection.count::<#struct_name, mystiko_protos::storage::v1::QueryFilter>(None).await
                }
                pub async fn update(&self, document: &mystiko_storage::Document<#struct_name>) ->
                    anyhow::Result<
                        mystiko_storage::Document<#struct_name>,
                        mystiko_storage::StorageError> {
                    self.collection.update(document).await
                }
                pub async fn update_batch(&self,
                    documents: &[mystiko_storage::Document<#struct_name>]) ->
                        anyhow::Result<
                            Vec<mystiko_storage::Document<#struct_name>>,
                            mystiko_storage::StorageError> {
                    self.collection.update_batch(documents).await
                }
                pub async fn update_all<V>(&self,
                    column_values: V) -> anyhow::Result<(), mystiko_storage::StorageError>
                where
                    V: Into<mystiko_storage::ColumnValues>
                {
                    self.collection.update_by_filter::<#struct_name, V, mystiko_protos::storage::v1::QueryFilter>(column_values, None).await
                }
                pub async fn update_by_filter<V, Q>(&self, column_values: V, filter: Q)
                    -> anyhow::Result<(), mystiko_storage::StorageError>
                where
                    V: Into<mystiko_storage::ColumnValues>,
                    Q: Into<mystiko_protos::storage::v1::QueryFilter>
                {
                    self.collection.update_by_filter::<#struct_name, V, Q>(column_values, Some(filter)).await
                }
                pub async fn delete(&self, document: &mystiko_storage::Document<#struct_name>) ->
                    anyhow::Result<(), mystiko_storage::StorageError> {
                    self.collection.delete(document).await
                }
                pub async fn delete_batch(&self,
                    documents: &[mystiko_storage::Document<#struct_name>]) ->
                        anyhow::Result<(), mystiko_storage::StorageError> {
                    self.collection.delete_batch(documents).await
                }
                pub async fn delete_all(&self) -> anyhow::Result<(), mystiko_storage::StorageError> {
                    self.collection.delete_by_filter::<#struct_name, mystiko_protos::storage::v1::QueryFilter>(None).await
                }
                pub async fn delete_by_filter<Q>(&self, filter: Q) ->
                    anyhow::Result<(), mystiko_storage::StorageError>
                where
                    Q: Into<mystiko_protos::storage::v1::QueryFilter>
                {
                    self.collection.delete_by_filter::<#struct_name,Q>(Some(filter)).await
                }
                pub async fn collection_exists(&self) ->
                    anyhow::Result<bool, mystiko_storage::StorageError> {
                    self.collection.collection_exists(&#struct_name::collection_name()).await
                }
                pub async fn migrate(&self) -> Result<
                        mystiko_storage::Document<mystiko_storage::MigrationHistory>,
                        mystiko_storage::StorageError> {
                    self.collection.migrate::<#struct_name>().await
                }
                pub fn formatter(&self) -> &F {
                    self.collection.formatter()
                }
                pub fn storage(&self) -> &S {
                    self.collection.storage()
                }
            }
        })
    }
}

impl StructAttribute {
    pub fn new(attrs: &[syn::Attribute]) -> Result<Self, syn::parse::Error> {
        let mut collection_name: Option<syn::Expr> = None;
        let mut unique_columns: Option<syn::Expr> = None;
        let mut index_columns: Option<syn::Expr> = None;
        let mut migrations: Option<syn::Expr> = None;
        for attr in attrs {
            if attr.path().is_ident("collection") {
                attr.parse_nested_meta(|meta| {
                    let expr: syn::Expr = meta.value()?.parse()?;
                    if meta.path.is_ident("name") {
                        collection_name = Some(expr);
                    } else if meta.path.is_ident("uniques") {
                        unique_columns = Some(expr);
                    } else if meta.path.is_ident("indexes") {
                        index_columns = Some(expr);
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
            index_columns,
            migrations,
        })
    }
}
