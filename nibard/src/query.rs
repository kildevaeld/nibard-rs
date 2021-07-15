use futures::Stream;
use nibard_connection::{DatabaseRow, Error, Executor};
use nibard_query::{build, Statement};
use nibard_shared::{Dialect, Value};
// use sqlx::query::QueryAs;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub struct Query<'a> {
    sql: Cow<'a, str>,
    values: Vec<Value>,
}

impl<'a> Query<'a> {
    pub fn fetch<'e, 'c: 'e, E: Executor<'c>>(
        &'a self,
        e: E,
    ) -> impl Stream<Item = Result<DatabaseRow, Error>> + 'e
    where
        'a: 'c,
    {
        e.fetch(&self.sql, &self.values)
    }

    pub async fn fetch_one<'e, 'c: 'e, E: Executor<'c>>(
        &'a self,
        e: E,
    ) -> Result<DatabaseRow, Error>
    where
        'a: 'c,
    {
        e.fetch_one(&self.sql, &self.values).await
    }
}

pub trait StatementQuery: Statement + Sized {
    fn to_query(self, dialect: Dialect) -> Query<'static> {
        let (sql, values) = build(dialect, self).unwrap();
        Query {
            sql: sql.into(),
            values,
        }
    }
}

impl<S> StatementQuery for S where S: Statement {}

// pub fn query<S: Statement>(stmt: S) -> Query<'static> {
//     let (sql, values) = build(stmt).unwrap();

//     Query { sql, values }
// }
