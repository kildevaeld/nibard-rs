use super::Dialect;
use std::fmt::{self, Write};

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    Char(u32),
    VarChar(u32),
    Text,
    SmallInt,
    Int,
    BigInt,
    Float(u16),
    Real,
    Bool,
    Binary,
    Date,
    DateTime,
    #[cfg(feature = "json")]
    JSON,
    Auto,
}

impl Type {
    pub fn is_auto(&self) -> bool {
        match self {
            Type::Auto => true,
            _ => false,
        }
    }

    fn to_pg(&self, out: &mut dyn Write) -> fmt::Result {
        match self {
            Type::Auto => out.write_str("SERIAL"),
            Type::BigInt => out.write_str("BIGINT"),
            Type::Binary => out.write_str("BYTEA"),
            Type::Bool => out.write_str("BOOL"),
            Type::Char(i) => write!(out, "CHAR({})", i),
            Type::VarChar(i) => write!(out, "VARCHAR({})", i),
            Type::Date => out.write_str("DATE"),
            Type::DateTime => out.write_str("TIMESTAMP"),
            Type::Float(p) => write!(out, "FLOAT({})", p),
            Type::Real => out.write_str("REAL"),
            Type::SmallInt => out.write_str("SMALLINT"),
            Type::Text => out.write_str("TEXT"),
            Type::Int => out.write_str("INTEGER"),
            #[cfg(feature = "json")]
            Type::JSON => out.write_str("JSON"),
        }
    }

    fn to_sqlite(&self, out: &mut dyn Write) -> fmt::Result {
        match self {
            Type::Auto => out.write_str("INTEGER"),
            Type::BigInt => out.write_str("BIGINT"),
            Type::Binary => out.write_str("BYTEA"),
            Type::Bool => out.write_str("BOOL"),
            Type::Char(i) => write!(out, "CHAR({})", i),
            Type::VarChar(i) => write!(out, "VARCHAR({})", i),
            Type::Date => out.write_str("DATE"),
            Type::DateTime => out.write_str("TIMESTAMP"),
            Type::Float(p) => write!(out, "FLOAT({})", p),
            Type::Real => out.write_str("REAL"),
            Type::SmallInt => out.write_str("SMALLINT"),
            Type::Text => out.write_str("TEXT"),
            Type::Int => out.write_str("INTEGER"),
            #[cfg(feature = "json")]
            Type::JSON => out.write_str("JSON"),
        }
    }

    fn to_mysql(&self, out: &mut dyn Write) -> fmt::Result {
        match self {
            Type::Auto => out.write_str("INTEGER"),
            Type::BigInt => out.write_str("BIGINT"),
            Type::Binary => out.write_str("BYTEA"),
            Type::Bool => out.write_str("BOOL"),
            Type::Char(i) => write!(out, "CHAR({})", i),
            Type::VarChar(i) => write!(out, "VARCHAR({})", i),
            Type::Date => out.write_str("DATE"),
            Type::DateTime => out.write_str("TIMESTAMP"),
            Type::Float(p) => write!(out, "FLOAT({})", p),
            Type::Real => out.write_str("REAL"),
            Type::SmallInt => out.write_str("SMALLINT"),
            Type::Text => out.write_str("TEXT"),
            Type::Int => out.write_str("INTEGER"),
            #[cfg(feature = "json")]
            Type::JSON => out.write_str("JSON"),
        }
    }

    pub fn write_sql(&self, out: &mut dyn fmt::Write, dialect: Dialect) -> fmt::Result {
        match dialect {
            Dialect::Pg => self.to_pg(out),
            Dialect::Sqlite => self.to_sqlite(out),
            Dialect::MySQL => self.to_mysql(out),
        }
    }
}
