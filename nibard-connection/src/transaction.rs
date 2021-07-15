pub use super::{
    error::*,
    executor::{Executor, QueryResult},
    row::DatabaseRow,
};
use futures::{
    future::{BoxFuture, FutureExt, TryFutureExt},
    stream::{BoxStream, StreamExt, TryStreamExt},
};
use nibard_shared::{Dialect, Value};
use sqlx::Executor as SqlxExecutor;

pub enum DatabaseTransaction<'c> {
    #[cfg(feature = "postgres")]
    Pg(sqlx::Transaction<'c, sqlx::Postgres>),
    #[cfg(feature = "sqlite")]
    Sqlite(sqlx::Transaction<'c, sqlx::Sqlite>),
    #[cfg(feature = "mysql")]
    MySQL(sqlx::Transaction<'c, sqlx::MySql>),
    #[cfg(all(
        not(feature = "sqlite"),
        not(feature = "mysql"),
        not(feature = "postgres")
    ))]
    _NoRuntime(&'c ()), // _Un(&'c ()),
}

impl<'c> DatabaseTransaction<'c> {
    pub fn dialect(&self) -> Dialect {
        match self {
            #[cfg(feature = "postgres")]
            DatabaseTransaction::Pg(_) => Dialect::Pg,
            #[cfg(feature = "sqlite")]
            DatabaseTransaction::Sqlite(_) => Dialect::Sqlite,
            #[cfg(feature = "mysql")]
            DatabaseTransaction::MySQL(_) => Dialect::MySQL,
        }
    }

    pub async fn commit(self) -> Result<(), Error> {
        match self {
            #[cfg(feature = "postgres")]
            DatabaseTransaction::Pg(pg) => {
                pg.commit().await?;
            }
            #[cfg(feature = "sqlite")]
            DatabaseTransaction::Sqlite(sqlite) => {
                sqlite.commit().await?;
            }
            #[cfg(feature = "mysql")]
            DatabaseTransaction::MySQL(mysql) => {
                mysql.commit().await?;
            }
        }

        Ok(())
    }
}

impl<'c, 'a> Executor<'a> for &'a mut DatabaseTransaction<'c> {
    fn dialect(&self) -> Dialect {
        (&**self).dialect()
    }
    fn fetch_one(
        self,
        query: &'a str,
        values: &'a [Value],
    ) -> futures::future::BoxFuture<'a, Result<DatabaseRow, Error>> {
        let fut = async move {
            let row = match self {
                #[cfg(feature = "postgres")]
                DatabaseTransaction::Pg(pg) => {
                    let q = bind_values!(values, sqlx::query(query));
                    q.fetch_one(pg).await.map(DatabaseRow::Pg)?
                }
                #[cfg(feature = "sqlite")]
                DatabaseTransaction::Sqlite(sqlite) => {
                    let q = bind_values!(values, sqlx::query(query));
                    q.fetch_one(sqlite).await.map(DatabaseRow::Sqlite)?
                }
                #[cfg(feature = "mysql")]
                DatabaseTransaction::MySQL(mysql) => {
                    let q = bind_values!(values, sqlx::query(query));
                    q.fetch_one(mysql).await.map(DatabaseRow::MySQL)?
                }
            };

            Ok(row)
        };

        Box::pin(fut)
    }

    fn fetch(
        self,
        query: &'a str,
        values: &'a [Value],
    ) -> BoxStream<'a, Result<DatabaseRow, Error>> {
        let row = match self {
            #[cfg(feature = "postgres")]
            DatabaseTransaction::Pg(pg) => {
                let q = bind_values!(values, sqlx::query(query));
                q.fetch(pg)
                    .map_ok(|pg| DatabaseRow::Pg(pg))
                    .err_into()
                    .boxed()
            }
            #[cfg(feature = "sqlite")]
            DatabaseTransaction::Sqlite(sqlite) => {
                let q = bind_values!(values, sqlx::query(query));
                q.fetch(sqlite)
                    .map_ok(|sqlite| DatabaseRow::Sqlite(sqlite))
                    .err_into()
                    .boxed()
            }
            #[cfg(feature = "mysql")]
            DatabaseTransaction::MySQL(mysql) => {
                let q = bind_values!(values, sqlx::query(query));
                q.fetch(mysql)
                    .map_ok(|mysql| DatabaseRow::MySQL(mysql))
                    .err_into()
                    .boxed()
            }
        };
        row
    }

    fn execute(
        self,
        query: &'a str,
        values: &'a [Value],
    ) -> BoxFuture<'a, Result<QueryResult, Error>> {
        match self {
            #[cfg(feature = "postgres")]
            DatabaseTransaction::Pg(pg) => {
                let q = bind_values!(values, sqlx::query(query));
                q.execute(pg)
                    .err_into()
                    .map_ok(|ret| QueryResult {
                        rows_affected: ret.rows_affected(),
                        last_insert_id: None,
                    })
                    .boxed()
            }
            #[cfg(feature = "sqlite")]
            DatabaseTransaction::Sqlite(sqlite) => {
                let q = bind_values!(values, sqlx::query(query));
                q.execute(sqlite)
                    .map_ok(|ret| QueryResult {
                        rows_affected: ret.rows_affected(),
                        last_insert_id: Some(ret.last_insert_rowid()),
                    })
                    .err_into()
                    .boxed()
            }
            #[cfg(feature = "mysql")]
            DatabaseTransaction::MySQL(mysql) => {
                let q = bind_values!(values, sqlx::query(query));
                q.execute(mysql)
                    .err_into()
                    .map_ok(|ret| QueryResult {
                        rows_affected: ret.rows_affected(),
                        last_insert_id: None,
                    })
                    .boxed()
            }
        }
    }

    fn execute_many(
        self,
        query: &'a str,
    ) -> BoxFuture<'a, BoxStream<'a, Result<QueryResult, Error>>> {
        async move {
            match self {
                #[cfg(feature = "postgres")]
                DatabaseTransaction::Pg(pg) => pg
                    .execute_many(query)
                    .err_into()
                    .map_ok(|ret| QueryResult {
                        rows_affected: ret.rows_affected(),
                        last_insert_id: None,
                    })
                    .boxed(),
                #[cfg(feature = "sqlite")]
                DatabaseTransaction::Sqlite(sqlite) => {
                    let q = sqlx::query(query);
                    q.execute_many(sqlite)
                        .await
                        .map_ok(|ret| QueryResult {
                            rows_affected: ret.rows_affected(),
                            last_insert_id: Some(ret.last_insert_rowid()),
                        })
                        .err_into()
                        .boxed()
                }
                #[cfg(feature = "mysql")]
                DatabaseTransaction::MySQL(mysql) => mysql
                    .execute_many(query)
                    .err_into()
                    .map_ok(|ret| QueryResult {
                        rows_affected: ret.rows_affected(),
                        last_insert_id: None,
                    })
                    .boxed(),
            }
        }
        .boxed()
    }
}
