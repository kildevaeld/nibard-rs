use super::{Expression, Select, Table};
use crate::{Context, Error};
use std::fmt::Write;

pub trait Joinable<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<'a, C: Context> Joinable<C> for Box<dyn Joinable<C> + 'a> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

pub struct Join<T> {
    kind: JoinType,
    table: T,
}

impl<T: Send + Sync, C: Context> Joinable<C> for Join<T>
where
    T: Table<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.kind.build(ctx)?;
        ctx.write_str(" ")?;
        self.table.build(ctx)?;
        // self.table.build(ctx)?;
        Ok(())
    }
}

impl<T> Join<T> {
    pub fn inner(table: T) -> Join<T> {
        Join {
            kind: JoinType::Inner,
            table,
        }
    }

    pub fn left(table: T) -> Join<T> {
        Join {
            kind: JoinType::Left,
            table,
        }
    }
    pub fn on<E>(self, e: E) -> JoinOn<T, E> {
        JoinOn { join: self, on: e }
    }
}

pub struct JoinOn<T, E> {
    join: Join<T>,
    on: E,
}

impl<T, E, C: Context> Joinable<C> for JoinOn<T, E>
where
    E: Expression<C> + Send + Sync,
    T: Table<C> + Send + Sync,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.join.build(ctx)?;
        ctx.write_str(" ON ")?;
        self.on.build(ctx)?;
        Ok(())
    }
}

pub enum JoinType {
    Inner,
    Left,
    Right,
    Outer,
}

impl JoinType {
    fn build<C: Context>(&self, ctx: &mut C) -> Result<(), Error> {
        match self {
            JoinType::Inner => {
                //
                ctx.write_str("INNER JOIN")
            }
            JoinType::Left => ctx.write_str("LEFT JOIN"),
            JoinType::Right => ctx.write_str("RIGHT JOIN"),
            JoinType::Outer => ctx.write_str("OUTER JOIN"),
        }?;
        Ok(())
    }
}
