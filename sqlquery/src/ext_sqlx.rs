use super::{build, Error, Expression, Statement};
use sqlquery_shared::{Dialect, Value};
use sqlx::{query::Query as _, Database, Executor};
use std::marker::PhantomData;

macro_rules! bind_value {
    ($val: expr, $query: expr) => {
        match $val {
            Value::Text(text) => $query.bind(text),
            Value::Int(i) => $query.bind(i),
            Value::SmallInt(i) => $query.bind(i),
            Value::BigInt(i) => $query.bind(i),
            Value::Bool(b) => $query.bind(b),
            Value::Date(date) => $query.bind(date),
            Value::DateTime(date) => $query.bind(date),
            Value::Binary(blob) => $query.bind(blob),
            // worm_types::Value::Null => $query.bind(None),
            _ => panic!("value not implemented {:?}", $val),
        }
    };
}

macro_rules! bind_values {
    ($values: expr, $query: expr) => {{
        let mut query = $query;
        for val in $values {
            query = bind_value!(val, query);
        }
        query
    }};
}

pub trait WithDialect {
    fn dialect(&self) -> &Dialect;
}

pub trait QueryConn<DB: Database>: Statement + Sized {
    fn dialect(&self) -> Dialect;
    fn query(self) -> Result<Query<DB>, Error> {
        let (sql, values) = build(self.dialect(), self)?;
        Ok(Query {
            _db: PhantomData,
            sql,
            values,
        })
    }
}

pub struct Query<DB: Database> {
    _db: PhantomData<DB>,
    sql: String,
    values: Vec<Value>,
}

impl<'q, DB: Database> Query<DB> {
    /// Execute the query and return the generated results as a stream.
    pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<O, Error>>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        // F: 'e,
        // O: 'e,
    {
        let mut sqlx = sqlx::query(&self.sql);
        bind_values!(self.values, sqlx);
        // for v in self.values {
        //     sqlx.bind(v);
        // }
        sqlx.fetch(executor)
    }
}

#[cfg(feature = "sqlx-sqlite")]
mod sqlite {
    use sqlx::sqlite::{Sqlite, SqliteConnection};

    impl<S> QueryConn<Sqlite> for S
    where
        S: Statement,
    {
        fn dialect() -> Dialect {
            Dialect::Sqlite
        }
    }
}
