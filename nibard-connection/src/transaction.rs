pub use super::{
    error::*,
    executor::{Execute, Executor, QueryResult},
    row::DatabaseRow,
};
use futures::{
    future::{BoxFuture, FutureExt, TryFutureExt},
    stream::{BoxStream, StreamExt, TryStreamExt},
};
use nibard_shared::{Dialect, Value};

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

impl<'c, 't> Executor<'c> for &'c mut DatabaseTransaction<'t> {
    fn dialect(&self) -> Dialect {
        (&**self).dialect()
    }
    fn fetch_one<'e, 'q, E>(
        self,
        execute: E,
    ) -> futures::future::BoxFuture<'e, Result<DatabaseRow, Error>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>,
    {
        let fut = async move {
            let row = match self {
                #[cfg(feature = "postgres")]
                DatabaseTransaction::Pg(pg) => {
                    let q = query_and_bind!(execute);
                    q.fetch_one(pg).await.map(DatabaseRow::Pg)?
                }
                #[cfg(feature = "sqlite")]
                DatabaseTransaction::Sqlite(sqlite) => {
                    let q = query_and_bind!(execute);
                    q.fetch_one(sqlite).await.map(DatabaseRow::Sqlite)?
                }
                #[cfg(feature = "mysql")]
                DatabaseTransaction::MySQL(mysql) => {
                    let q = query_and_bind!(execute);
                    q.fetch_one(mysql).await.map(DatabaseRow::MySQL)?
                }
            };

            Ok(row)
        };

        Box::pin(fut)
    }

    fn fetch<'e, 'q, E>(self, execute: E) -> BoxStream<'e, Result<DatabaseRow, Error>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>,
    {
        let row = match self {
            #[cfg(feature = "postgres")]
            DatabaseTransaction::Pg(pg) => {
                let q = query_and_bind!(execute);
                q.fetch(pg)
                    .map_ok(|pg| DatabaseRow::Pg(pg))
                    .err_into()
                    .boxed()
            }
            #[cfg(feature = "sqlite")]
            DatabaseTransaction::Sqlite(sqlite) => {
                let q = query_and_bind!(execute);
                q.fetch(sqlite)
                    .map_ok(|sqlite| DatabaseRow::Sqlite(sqlite))
                    .err_into()
                    .boxed()
            }
            #[cfg(feature = "mysql")]
            DatabaseTransaction::MySQL(mysql) => {
                let q = query_and_bind!(execute);
                q.fetch(mysql)
                    .map_ok(|mysql| DatabaseRow::MySQL(mysql))
                    .err_into()
                    .boxed()
            }
        };
        row
    }

    fn execute<'e, 'q, E>(self, execute: E) -> BoxFuture<'e, Result<QueryResult, Error>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>,
    {
        match self {
            #[cfg(feature = "postgres")]
            DatabaseTransaction::Pg(pg) => {
                let q = query_and_bind!(execute);
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
                let q = query_and_bind!(execute);
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
                let q = query_and_bind!(execute);
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

    fn execute_many<'e, 'q, E>(
        self,
        execute: E,
    ) -> BoxFuture<'e, BoxStream<'e, Result<QueryResult, Error>>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>,
    {
        async move {
            match self {
                #[cfg(feature = "postgres")]
                DatabaseTransaction::Pg(pg) => {
                    let q = query_and_bind!(execute);
                    q.execute_many(pg)
                        .await
                        .map_ok(|ret| QueryResult {
                            rows_affected: ret.rows_affected(),
                            last_insert_id: None,
                        })
                        .err_into()
                        .boxed()
                }
                #[cfg(feature = "sqlite")]
                DatabaseTransaction::Sqlite(sqlite) => {
                    let q = query_and_bind!(execute);
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
                DatabaseTransaction::MySQL(mysql) => {
                    let q = query_and_bind!(execute);
                    q.execute_many(mysql)
                        .await
                        .map_ok(|ret| QueryResult {
                            rows_affected: ret.rows_affected(),
                            last_insert_id: None,
                        })
                        .err_into()
                        .boxed()
                }
            }
        }
        .boxed()
    }
}
