use super::{Expression, Table};
use crate::{Context, Error};
use std::fmt::Write;

pub trait Joinable {
    fn build(&self, ctx: &mut Context) -> Result<(), Error>;
}

pub struct Join<T> {
    kind: JoinType,
    table: T,
}

impl<T: Send + Sync> Joinable for Join<T>
where
    T: Table,
{
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
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

impl<T, E> Joinable for JoinOn<T, E>
where
    E: Expression + Send + Sync,
    T: Table + Send + Sync,
{
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
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
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
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
