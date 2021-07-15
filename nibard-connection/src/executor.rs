use super::error::*;
use super::row::DatabaseRow;
use futures::{future::BoxFuture, stream::BoxStream};
use nibard_shared::{Dialect, Value};

pub struct QueryResult {
    pub rows_affected: u64,
    pub last_insert_id: Option<i64>,
}

pub trait Executor<'a> {
    fn dialect(&self) -> Dialect;

    fn fetch_one(
        self,
        query: &'a str,
        values: &'a [Value],
    ) -> futures::future::BoxFuture<'a, Result<DatabaseRow, Error>>;

    fn fetch(
        self,
        query: &'a str,
        values: &'a [Value],
    ) -> BoxStream<'a, Result<DatabaseRow, Error>>;

    fn execute(
        self,
        query: &'a str,
        values: &'a [Value],
    ) -> BoxFuture<'a, Result<QueryResult, Error>>;

    fn execute_many(
        self,
        query: &'a str,
    ) -> BoxFuture<'a, BoxStream<'a, Result<QueryResult, Error>>>;
}

pub trait Execute<'q> {
    fn sql(&self) -> &'q str;
}
