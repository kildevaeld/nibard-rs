use async_stream::stream;
use futures::Stream;
use nibard_connection::{DatabaseRow, Error, Execute, Executor, QueryResult};
use nibard_dsl::{build, DefaultContext, Statement};
use nibard_shared::{Dialect, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    sql: String,
    values: Vec<Value>,
}

impl Query {
    pub fn new(sql: String, values: Vec<Value>) -> Query {
        Query { sql, values }
    }
}

impl Query {
    pub fn fetch<'e, 'c: 'e, E: Executor<'c>>(
        self,
        e: E,
    ) -> impl Stream<Item = Result<DatabaseRow, Error>> + 'e
    where
        E: 'c,
    {
        stream! {
            let stream = e.fetch(&self);

            for await value in stream {
                yield value
            }
        }
    }

    pub async fn fetch_one<'e, E: Executor<'e>>(self, e: E) -> Result<DatabaseRow, Error>
    where
        Self: 'e,
    {
        e.fetch_one(&self).await
    }

    pub async fn execute<'e, E: Executor<'e>>(self, e: E) -> Result<QueryResult, Error>
    where
        Self: 'e,
    {
        e.execute(&self).await
    }
}

impl<'q> Execute<'q> for &'q Query {
    fn sql(&self) -> &'q str {
        &self.sql
    }

    fn args(&self) -> Option<&'q [Value]> {
        Some(&self.values)
    }
}

pub trait StatementQuery: Statement<DefaultContext> + Sized {
    fn to_query(self, dialect: Dialect) -> Query {
        let (sql, values) = build(dialect, self).unwrap();
        Query { sql, values }
    }
}

impl<S> StatementQuery for S where S: Statement<DefaultContext> {}

// pub fn query<S: Statement>(stmt: S) -> Query<'static> {
//     let (sql, values) = build(stmt).unwrap();

//     Query { sql, values }
// }
