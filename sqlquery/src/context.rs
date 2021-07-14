use super::{Error, Statement};
use sqlquery_shared::{Dialect, Value};
use std::fmt::{self, Write as _};

pub struct Context<'a> {
    data: &'a mut dyn fmt::Write,
    dialect: Dialect,
    values: Vec<Value>,
}

impl<'a> Context<'a> {
    pub fn new(dialect: Dialect, writer: &'a mut dyn fmt::Write) -> Context<'a> {
        Context {
            dialect,
            data: writer,
            values: Vec::default(),
        }
    }
    pub fn push(&mut self, value: Value) -> Result<&mut Self, Error> {
        if value == Value::Null {
            self.write_str("NULL")?;
        } else {
            self.values.push(value);
            match self.dialect {
                Dialect::MySQL => self.write_str("?"),
                Dialect::Sqlite => self.write_str("?"),
                Dialect::Pg => {
                    write!(self.data, "${}", self.values.len())
                }
            }?;
        }

        Ok(self)
    }

    pub fn build<S: Statement>(mut self, stmt: S) -> Result<Vec<Value>, Error> {
        stmt.build(&mut self)?;
        Ok(self.values)
    }
}

pub fn build<S: Statement>(dialect: Dialect, stmt: S) -> Result<(String, Vec<Value>), Error> {
    let mut output = String::default();
    let values = {
        let mut ctx = Context::new(dialect, &mut output);
        stmt.build(&mut ctx)?;
        ctx.values
    };
    Ok((output, values))
}

impl<'a> fmt::Write for Context<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.data.write_str(s)
    }
}
