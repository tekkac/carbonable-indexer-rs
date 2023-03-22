use deadpool_postgres::Pool;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_postgres::PostgresBinder;
use std::sync::Arc;

use super::{
    entity::{Uri, UriIden},
    PostgresError,
};

#[derive(Debug)]
pub struct PostgresUri {
    pub db_client_pool: Arc<Pool>,
}

impl PostgresUri {
    pub fn new(db_client_pool: Arc<Pool>) -> Self {
        Self { db_client_pool }
    }

    pub async fn find_by_uri(&self, uri: &str) -> Result<Option<Uri>, PostgresError> {
        let (sql, params) = Query::select()
            .column(UriIden::Id)
            .column(UriIden::Uri)
            .column(UriIden::Data)
            .from(UriIden::Table)
            .and_where(Expr::col(UriIden::Uri).eq(uri))
            .build_postgres(PostgresQueryBuilder);
        match self
            .db_client_pool
            .get()
            .await?
            .query_one(sql.as_str(), &params.as_params())
            .await
        {
            Ok(v) => Ok(Some(v.into())),
            Err(_) => Ok(None),
        }
    }

    pub async fn create(&self, uri: &str, data: serde_json::Value) -> Result<Uri, PostgresError> {
        let client = self.db_client_pool.get().await?;
        let id = uuid::Uuid::new_v4();

        let (sql, values) = Query::insert()
            .into_table(UriIden::Table)
            .columns([UriIden::Id, UriIden::Uri, UriIden::Data])
            .values([id.into(), uri.into(), data.clone().into()])?
            .build_postgres(PostgresQueryBuilder);
        let _res = client.execute(sql.as_str(), &values.as_params()).await?;
        Ok(Uri {
            id,
            uri: uri.to_string(),
            data,
        })
    }
}