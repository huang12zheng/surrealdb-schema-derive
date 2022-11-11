use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn gen_fn_fetch_from_datastore(table_name: &Ident) -> TokenStream {
    quote! {
        async fn fetch_from_datastore(id: u64) -> surrealdb_schema_derive::anyhow::Result<Option<Self::Row>> {
            let query = surrealdb_schema_derive::runtime::make_lookup_query(
                stringify!(#table_name).into(),
                surrealdb_schema_derive::surrealdb::sql::Id::from(id),
            );
            let db = DBX.get().unwrap();

            Ok(surrealdb_schema_derive::runtime::run_single_row_query(&db.datastore, &db.session, query).await?)
        }
    }
}

pub(crate) fn gen_fn_save_to_datastore(table_name: &Ident) -> TokenStream {
    quote! {
        async fn save_to_datastore(self) -> surrealdb_schema_derive::anyhow::Result<Self::Row> {
            let query = surrealdb_schema_derive::surrealdb::sql::Query(surrealdb_schema_derive::surrealdb::sql::Statements(vec![
                surrealdb_schema_derive::surrealdb::sql::Statement::Insert(surrealdb_schema_derive::surrealdb::sql::statements::InsertStatement {
                    into: surrealdb_schema_derive::surrealdb::sql::Table(stringify!(#table_name).into()),
                    data: surrealdb_schema_derive::runtime::to_single_row_values_expression(self.into())?,
                    ignore: false,
                    update: None,
                    output: Some(surrealdb_schema_derive::surrealdb::sql::Output::After),
                    timeout: None,
                    parallel: false,
                }),
            ]));
            let db = DBX.get().unwrap();
            let row = surrealdb_schema_derive::runtime::run_single_row_query(&db.datastore, &db.session, query).await;
            Ok(row?.unwrap())
        }
    }
}

pub(crate) fn gen_fn_delete_to_datastore(table_name: &Ident) -> TokenStream {
    quote! {
        async fn delete_to_datastore(id: u64) -> bool {
            let query = surrealdb_schema_derive::surrealdb::sql::Query(surrealdb_schema_derive::surrealdb::sql::Statements(vec![
                surrealdb_schema_derive::surrealdb::sql::Statement::Delete(surrealdb_schema_derive::surrealdb::sql::statements::DeleteStatement {
                    what: Values(vec![Value::Thing(Thing {
                        tb: stringify!(#table_name).into(),
                        id: surrealdb_schema_derive::surrealdb::sql::Id::from(id),
                    })]),
                    cond: None,
                    output: None,
                    timeout: None,
                    parallel: false,
                }),
            ]));
            let db = DBX.get().unwrap();

            surrealdb_schema_derive::runtime::run_single_row_query::<Self>(&db.datastore, &db.session, query).await.is_ok()
        }
    }
}
