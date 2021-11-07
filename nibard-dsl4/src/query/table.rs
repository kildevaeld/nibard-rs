use super::Column;
use crate::{Context, Error};
use std::borrow::Cow;
use std::fmt::Write;

pub trait Table<C: Context> {
    fn name(&self) -> &str;
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        Ok(ctx.write_str(self.name())?)
    }
}

pub trait TableExt<CTX: Context>: Table<CTX> + Sized {
    fn alias<'a>(self, name: impl Into<Cow<'a, str>>) -> TableAlias<'a, Self> {
        TableAlias {
            table: self,
            alias: name.into(),
        }
    }

    fn col<C: Column<CTX>>(self, name: C) -> TableCol<Self, C> {
        TableCol {
            table: self,
            column: name,
        }
    }
}

impl<T: Table<C>, C: Context> TableExt<C> for T where T: Table<C> {}

impl<T, C: Context> Table<C> for &T
where
    T: Table<C>,
{
    fn name(&self) -> &str {
        (&**self).name()
    }

    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

impl<'a, C: Context> Table<C> for &'a str {
    fn name(&self) -> &str {
        self
    }
}

impl<C: Context> Table<C> for String {
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

impl<'a, T, C: Context> Table<C> for TableAlias<'a, T>
where
    T: Table<C>,
{
    fn name(&self) -> &str {
        &self.alias
    }

    fn build(&self, ctx: &mut C) -> Result<(), Error> {
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
// where
//     T: Table<CTX>,
//     C: Column<CTX>,
{
    pub fn column(&self) -> &C {
        &self.column
    }

    pub fn table(&self) -> &T {
        &self.table
    }
}

impl<T, C, CTX: Context> Column<CTX> for TableCol<T, C>
where
    T: Table<CTX>,
    C: Column<CTX>,
{
    fn name(&self) -> &str {
        self.column().name()
    }
    fn build(&self, ctx: &mut CTX) -> Result<(), Error> {
        write!(ctx, "{}.", self.table.name())?;
        self.column().build(ctx)?;
        Ok(())
    }
}

pub fn table<C: Context, T: Table<C>>(table: T) -> impl Table<C> {
    table
}
