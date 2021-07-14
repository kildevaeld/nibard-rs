use super::Error;
#[cfg(feature = "time")]
use chrono::NaiveDateTime;
use sqlquery_shared::Value;
use sqlx::{Column as _, Row as SqlxRow, TypeInfo, ValueRef as SqlxValueRef};

pub enum DatabaseRow {
    #[cfg(feature = "postgres")]
    Pg(sqlx::postgres::PgRow),
    #[cfg(feature = "sqlite")]
    Sqlite(sqlx::sqlite::SqliteRow),
    #[cfg(feature = "mysql")]
    MySQL(sqlx::mysql::MySqlRow),
}

pub struct Column<'a> {
    name: &'a str,
}

pub trait Row {
    fn try_get(&self, name: &str) -> Result<Value, Error>;
    fn columns<'a>(&'a self) -> Vec<Column<'a>>;
}

pub trait RowExt: Row {
    fn to_map(&self) -> std::collections::HashMap<String, Value> {
        let mut out = std::collections::HashMap::default();

        for col in self.columns() {
            out.insert(col.name.to_owned(), self.try_get(col.name).unwrap());
        }
        out
    }
}

impl<R: Row> RowExt for R {}

impl Row for DatabaseRow {
    fn try_get(&self, name: &str) -> Result<Value, Error> {
        match self {
            #[cfg(feature = "postgres")]
            DatabaseRow::Pg(pg) => <sqlx::postgres::PgRow as Row>::try_get(pg, name),
            #[cfg(feature = "sqlite")]
            DatabaseRow::Sqlite(sqlite) => <sqlx::sqlite::SqliteRow as Row>::try_get(sqlite, name),
            #[cfg(feature = "mysql")]
            DatabaseRow::MySQL(mysql) => <sqlx::mysql::MySqlRow as Row>::try_get(mysql, name),
        }
    }

    fn columns<'a>(&'a self) -> Vec<Column<'a>> {
        match self {
            #[cfg(feature = "postgres")]
            DatabaseRow::Pg(pg) => <sqlx::postgres::PgRow as Row>::columns(pg),
            #[cfg(feature = "sqlite")]
            DatabaseRow::Sqlite(sqlite) => <sqlx::sqlite::SqliteRow as Row>::columns(sqlite),
            #[cfg(feature = "mysql")]
            DatabaseRow::MySQL(mysql) => <sqlx::mysql::MySqlRow as Row>::columns(mysql),
        }
    }
}

#[cfg(feature = "postgres")]
impl Row for sqlx::postgres::PgRow {
    fn try_get(&self, name: &str) -> Result<Value, Error> {
        let value_ref = self.try_get_raw(name)?;
        let type_info = value_ref.type_info();

        if value_ref.is_null() {
            return Ok(Value::Null);
        }

        let v = match type_info.name() {
            "INT4" => {
                let v: i32 = <Self as sqlx::Row>::try_get(self, name)?;
                Value::Int(v)
            }
            "TEXT" => {
                let v: String = <Self as sqlx::Row>::try_get(self, name)?;
                Value::Text(v)
            }
            "BYTEA" => {
                let v: Vec<u8> = <Self as sqlx::Row>::try_get(self, name)?;
                Value::Binary(v)
            }
            _ => {
                unimplemented!("type not implemented {}", type_info.name());
            }
        };

        Ok(v)
    }

    fn columns<'a>(&'a self) -> Vec<Column<'a>> {
        <Self as sqlx::Row>::columns(self)
            .iter()
            .map(|col| Column { name: col.name() })
            .collect::<Vec<_>>()
    }
}

#[cfg(feature = "sqlite")]
impl Row for sqlx::sqlite::SqliteRow {
    fn try_get(&self, name: &str) -> Result<Value, Error> {
        let value_ref = self.try_get_raw(name)?;
        let type_info = value_ref.type_info();

        if value_ref.is_null() {
            return Ok(Value::Null);
        }

        let v = match type_info.name() {
            "TEXT" => {
                let v: String = <Self as sqlx::Row>::try_get(self, name)?;
                Value::Text(v)
            }
            #[cfg(feature = "time")]
            "DATETIME" => {
                let v: NaiveDateTime = <Self as SqlxRow>::try_get(self, name)?;
                Value::DateTime(v)
            }
            "INTEGER" => {
                let v: i32 = <Self as sqlx::Row>::try_get(self, name)?;
                Value::Int(v)
            }
            "FLOAT" => {
                //
                let v: f64 = <Self as sqlx::Row>::try_get(self, name)?;
                Value::Float(v)
            }
            "BLOB" => {
                let v: Vec<u8> = <Self as sqlx::Row>::try_get(self, name)?;
                Value::Binary(v)
            }
            _ => {
                unimplemented!("type not implemented: {}", type_info.name());
            }
        };

        Ok(v)
    }
    fn columns<'a>(&'a self) -> Vec<Column<'a>> {
        <Self as sqlx::Row>::columns(self)
            .iter()
            .map(|col| Column { name: col.name() })
            .collect::<Vec<_>>()
    }
}

#[cfg(feature = "mysql")]
impl Row for sqlx::mysql::MySqlRow {
    fn try_get(&self, name: &str) -> Result<Value, Error> {
        unimplemented!("")
    }

    fn columns<'a>(&'a self) -> Vec<Column<'a>> {
        <Self as sqlx::Row>::columns(self)
            .iter()
            .map(|col| Column { name: col.name() })
            .collect::<Vec<_>>()
    }
}
