pub use super::{error::*, executor::*, row::*, transaction::*};
use futures::{
    future::BoxFuture, stream::BoxStream, FutureExt, StreamExt, TryFutureExt, TryStreamExt,
};
use nibard_shared::{Dialect, Value};
use std::str::FromStr;

pub enum ConnectOptions {
    #[cfg(feature = "postgres")]
    Pg(sqlx::postgres::PgConnectOptions),
    #[cfg(feature = "sqlite")]
    Sqlite(sqlx::sqlite::SqliteConnectOptions),
    #[cfg(feature = "mysql")]
    MySQL(sqlx::mysql::MySqlConnectOptions),
}

impl ConnectOptions {
    #[allow(unreachable_code)]
    async fn build(self) -> Result<DatabaseKind, Error> {
        let kind = match self {
            #[cfg(feature = "postgres")]
            ConnectOptions::Pg(p) => {
                sqlx::postgres::PgPoolOptions::new()
                    .connect_with(p)
                    .map_ok(DatabaseKind::Pg)
                    .await?
            }
            #[cfg(feature = "sqlite")]
            ConnectOptions::Sqlite(p) => {
                sqlx::sqlite::SqlitePoolOptions::new()
                    .connect_with(p)
                    .map_ok(DatabaseKind::Sqlite)
                    .await?
            }
            #[cfg(feature = "mysql")]
            ConnectOptions::MySQL(p) => {
                sqlx::mysql::MySqlPoolOptions::new()
                    .connect_with(p)
                    .map_ok(DatabaseKind::MySQL)
                    .await?
            }
        };

        Ok(kind)
    }

    pub async fn open(self) -> Result<Database, Error> {
        let kind = self.build().await?;
        Ok(Database { kind })
    }
}

fn get_dialect(n: &str) -> Result<Dialect, Error> {
    let d = match n {
        _ if n.starts_with("postgres:") || n.starts_with("postgresql:") => Dialect::Pg,
        _ if n.starts_with("sqlite:") => Dialect::Sqlite,
        _ if n.starts_with("mysql:") => Dialect::MySQL,
        _ => {
            panic!("Unknown connection string: {}", n)
        }
    };

    Ok(d)
}

impl FromStr for ConnectOptions {
    type Err = Error;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        let dialect = get_dialect(url)?;
        let c = match dialect {
            #[cfg(feature = "postgres")]
            Dialect::Pg => {
                sqlx::postgres::PgConnectOptions::from_str(url).map(ConnectOptions::Pg)?
            }

            #[cfg(feature = "mysql")]
            Dialect::MySQL => {
                sqlx::mysql::MySqlConnectOptions::from_str(url).map(ConnectOptions::MySQL)?
            }
            #[cfg(feature = "sqlite")]
            Dialect::Sqlite => {
                sqlx::sqlite::SqliteConnectOptions::from_str(url).map(ConnectOptions::Sqlite)?
            }
            _ => {
                panic!("dialect not found: {}", dialect);
            }
        };

        Ok(c)
    }
}

#[derive(Clone, Debug)]
pub enum DatabaseKind {
    #[cfg(feature = "postgres")]
    Pg(sqlx::PgPool),
    #[cfg(feature = "sqlite")]
    Sqlite(sqlx::SqlitePool),
    #[cfg(feature = "mysql")]
    MySQL(sqlx::MySqlPool),
}

impl DatabaseKind {
    pub async fn begin<'c>(&'c self) -> Result<DatabaseTransaction<'c>, Error> {
        match self {
            #[cfg(feature = "postgres")]
            DatabaseKind::Pg(pg) => Ok(pg.begin().await.map(DatabaseTransaction::Pg)?),
            #[cfg(feature = "sqlite")]
            DatabaseKind::Sqlite(sqlite) => {
                Ok(sqlite.begin().await.map(DatabaseTransaction::Sqlite)?)
            }
            #[cfg(feature = "mysql")]
            DatabaseKind::MySQL(mysql) => {
                Ok(mysql.begin().await.map(DatabaseTransaction::MySQL)?)
            }
        }
    }

    pub fn dialect(&self) -> Dialect {
        match self {
            #[cfg(feature = "postgres")]
            DatabaseKind::Pg(_) => Dialect::Pg,
            #[cfg(feature = "sqlite")]
            DatabaseKind::Sqlite(_) => Dialect::Sqlite,
            #[cfg(feature = "mysql")]
            DatabaseKind::MySQL(_) => Dialect::MySQL,
        }
    }
}

impl<'c> Executor<'c> for &'c DatabaseKind {
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
                DatabaseKind::Pg(pg) => {
                    let mut q = sqlx::query(execute.sql());
                    if let Some(values) = execute.args() {
                        q = bind_values!(values, q);
                    }
                    q.fetch_one(pg).await.map(DatabaseRow::Pg)?
                }
                #[cfg(feature = "mysql")]
                DatabaseKind::MySQL(pg) => {
                    let mut q = sqlx::query(execute.sql());
                    if let Some(values) = execute.args() {
                        q = bind_values!(values, q);
                    }
                    q.fetch_one(pg).await.map(DatabaseRow::MySQL)?
                }
                #[cfg(feature = "sqlite")]
                DatabaseKind::Sqlite(sqlite) => {
                    let mut q = sqlx::query(execute.sql());
                    if let Some(values) = execute.args() {
                        q = bind_values!(values, q);
                    }
                    q.fetch_one(sqlite).await.map(DatabaseRow::Sqlite)?
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
            DatabaseKind::Pg(pg) => {
                let q = query_and_bind!(execute);
                q.fetch(pg)
                    .map_ok(|pg| DatabaseRow::Pg(pg))
                    .err_into()
                    .boxed()
            }
            #[cfg(feature = "sqlite")]
            DatabaseKind::Sqlite(sqlite) => {
                let q = query_and_bind!(execute);
                q.fetch(sqlite)
                    .map_ok(|sqlite| DatabaseRow::Sqlite(sqlite))
                    .err_into()
                    .boxed()
            }
            #[cfg(feature = "mysql")]
            DatabaseKind::MySQL(pg) => {
                let q = query_and_bind!(execute);
                q.fetch(pg)
                    .map_ok(|pg| DatabaseRow::MySQL(pg))
                    .err_into()
                    .boxed()
            }
        };
        row
        //Box::pin(row.err_into())
    }
    fn execute<'e, 'q, E>(self, execute: E) -> BoxFuture<'e, Result<QueryResult, Error>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>,
    {
        match self {
            #[cfg(feature = "postgres")]
            DatabaseKind::Pg(pg) => {
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
            DatabaseKind::Sqlite(sqlite) => {
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
            DatabaseKind::MySQL(mysql) => {
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
                DatabaseKind::Pg(pg) => {
                    let q = query_and_bind!(execute);
                    q.execute_many(pg)
                        .await
                        .err_into()
                        .map_ok(|ret| QueryResult {
                            rows_affected: ret.rows_affected(),
                            last_insert_id: None,
                        })
                        .boxed()
                }
                #[cfg(feature = "sqlite")]
                DatabaseKind::Sqlite(sqlite) => {
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
                DatabaseKind::MySQL(mysql) => {
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

#[derive(Clone)]
pub struct Database {
    pub(crate) kind: DatabaseKind,
}

impl Database {
    pub async fn open(string: &str) -> Result<Database, Error> {
        let cfg = ConnectOptions::from_str(string)?;

        let kind = cfg.build().await?;

        Ok(Database { kind })
    }

    pub fn dialect(&self) -> Dialect {
        self.kind.dialect()
    }

    pub async fn begin<'c>(&'c self) -> Result<DatabaseTransaction<'c>, Error> {
        self.kind.begin().await
    }
}

impl<'c> Executor<'c> for &'c Database {
    fn dialect(&self) -> Dialect {
        self.kind.dialect()
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
        self.kind.fetch_one(execute)
    }

    fn fetch<'e, 'q, E>(self, execute: E) -> BoxStream<'e, Result<DatabaseRow, Error>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>,
    {
        self.kind.fetch(execute)
    }

    fn execute<'e, 'q, E>(self, e: E) -> BoxFuture<'e, Result<QueryResult, Error>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>,
    {
        self.kind.execute(e)
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
        self.kind.execute_many(execute)
    }
}
