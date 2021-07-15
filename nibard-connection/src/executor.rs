use super::error::*;
use super::row::DatabaseRow;
use futures::{future::BoxFuture, stream::BoxStream};
use nibard_shared::{Dialect, Value};

pub struct QueryResult {
    pub rows_affected: u64,
    pub last_insert_id: Option<i64>,
}

pub trait Executor<'c> {
    fn dialect(&self) -> Dialect;

    fn fetch_one<'e, 'q, E>(self, execute: E) -> BoxFuture<'e, Result<DatabaseRow, Error>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>;

    fn fetch<'e, 'q, E>(self, execute: E) -> BoxStream<'e, Result<DatabaseRow, Error>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>;

    fn execute<'e, 'q, E>(self, e: E) -> BoxFuture<'e, Result<QueryResult, Error>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>;

    fn execute_many<'e, 'q, E>(
        self,
        e: E,
    ) -> BoxFuture<'e, BoxStream<'e, Result<QueryResult, Error>>>
    where
        'q: 'e,
        'c: 'e,
        E: 'q + Execute<'q>;
}

pub trait Execute<'q>: Send {
    fn sql(&self) -> &'q str;
    fn args(&self) -> Option<&'q [Value]> {
        None
    }
}

impl<'q> Execute<'q> for &'q str {
    fn sql(&self) -> &'q str {
        self
    }
}

impl<'q> Execute<'q> for (&'q str, &'q [Value]) {
    fn sql(&self) -> &'q str {
        self.0
    }
    fn args(&self) -> Option<&'q [Value]> {
        Some(&self.1)
    }
}
