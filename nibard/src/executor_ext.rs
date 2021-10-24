use super::query::StatementQuery;
use futures::{
    future::{BoxFuture, FutureExt},
    stream::{BoxStream, StreamExt},
};
use nibard_connection::{DatabaseRow, Error, Executor, QueryResult};
use nibard_dsl::Statement;

pub trait ExecutorExt<'c>: Executor<'c> + Send {
    fn exec<S: Statement>(self, stmt: S) -> BoxFuture<'c, Result<QueryResult, Error>>
    where
        Self: Sized + 'c,
    {
        stmt.to_query(self.dialect()).execute(self).boxed()
    }

    fn query<S: Statement>(self, stmt: S) -> BoxStream<'c, Result<DatabaseRow, Error>>
    where
        Self: Sized + 'c,
    {
        stmt.to_query(self.dialect()).fetch(self).boxed()
    }

    fn query_one<S: Statement>(self, stmt: S) -> BoxFuture<'c, Result<DatabaseRow, Error>>
    where
        Self: Sized + 'c,
    {
        stmt.to_query(self.dialect()).fetch_one(self).boxed()
    }
}

impl<'c, E> ExecutorExt<'c> for E where E: Executor<'c> + Send {}
