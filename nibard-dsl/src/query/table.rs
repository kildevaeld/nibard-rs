use super::Column;
use crate::{Context, Error};
use std::borrow::Cow;
use std::fmt::Write;

pub trait Table {
    fn name(&self) -> &str;
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        Ok(ctx.write_str(self.name())?)
    }
}

pub trait TableExt: Table + Sized {
    fn alias<'a>(self, name: impl Into<Cow<'a, str>>) -> TableAlias<'a, Self> {
        TableAlias {
            table: self,
            alias: name.into(),
        }
    }

    fn col<C: Column>(self, name: C) -> TableCol<Self, C> {
        TableCol {
            table: self,
            column: name,
        }
    }
}

impl<T: Table> TableExt for T where T: Table {}

impl<T> Table for &T
where
    T: Table,
{
    fn name(&self) -> &str {
        (&**self).name()
    }

    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

impl<'a> Table for &'a str {
    fn name(&self) -> &str {
        self
    }
}

impl Table for String {
    fn name(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TableAlias<'a, T> {
    table: T,
    alias: Cow<'a, str>,
}

impl<'a, T> TableAlias<'a, T> {
    pub fn new(table: T, alias: impl Into<Cow<'a, str>>) -> TableAlias<'a, T> {
        TableAlias {
            table,
            alias: alias.into(),
        }
    }
}

impl<'a, T> TableAlias<'a, T> {
    pub fn table(&self) -> &T {
        &self.table
    }
    pub fn alias(&self) -> &str {
        &self.alias
    }
}

impl<'a, T> Table for TableAlias<'a, T>
where
    T: Table,
{
    fn name(&self) -> &str {
        &self.alias
    }

    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        self.table.build(ctx)?;
        write!(ctx, " AS {}", self.alias)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TableCol<T, C> {
    table: T,
    column: C,
}

impl<T, C> TableCol<T, C>
where
    T: Table,
    C: Column,
{
    pub fn column(&self) -> &C {
        &self.column
    }

    pub fn table(&self) -> &T {
        &self.table
    }
}

impl<T, C> Column for TableCol<T, C>
where
    T: Table,
    C: Column,
{
    fn name(&self) -> &str {
        self.column().name()
    }
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        write!(ctx, "{}.", self.table.name())?;
        self.column().build(ctx)?;
        Ok(())
    }
}

pub fn table<T: Table>(table: T) -> impl Table {
    table
}
