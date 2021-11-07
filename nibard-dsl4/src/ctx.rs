use super::error::Error;
use nibard_shared::{Dialect, Value};
use std::fmt::{self, Write};

pub trait Context: fmt::Write {
    fn dialect(&self) -> &Dialect;
    fn push(&mut self, value: Value) -> Result<&mut Self, Error>;

    fn build(self) -> Result<(String, Vec<Value>), Error>;
}

pub struct DefaultContext(Dialect, Vec<Value>, String);

impl fmt::Write for DefaultContext {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.2.write_str(s)
    }
}

impl Context for DefaultContext {
    fn dialect(&self) -> &Dialect {
        &self.0
    }
    fn push(&mut self, value: Value) -> Result<&mut Self, Error> {
        if value == Value::Null {
            self.write_str("NULL")?;
        } else {
            self.1.push(value);
            match self.0 {
                Dialect::MySQL => self.write_str("?"),
                Dialect::Sqlite => self.write_str("?"),
                Dialect::Pg => {
                    write!(self.2, "${}", self.1.len())
                }
            }?;
        }

        Ok(self)
    }

    fn build(self) -> Result<(String, Vec<Value>), Error> {
        Ok((self.2, self.1))
    }
}

pub fn build<S: crate::Statement<DefaultContext>>(
    dialect: Dialect,
    stmt: S,
) -> Result<(String, Vec<Value>), Error> {
    let mut ctx = DefaultContext(dialect, Vec::default(), String::default());
    stmt.build(&mut ctx)?;
    Ok(ctx.build()?)
}
