use crate::query::{Expression, IntoValue};
use crate::{Context, Error, Statement};
use std::borrow::Cow;
use std::fmt::Write;
use std::marker::PhantomData;

pub struct Delete<'a> {
    table: Cow<'a, str>,
}

impl<'a> Delete<'a> {
    pub fn new(table: impl Into<Cow<'a, str>>) -> Delete<'a> {
        Delete {
            table: table.into(),
        }
    }

    pub fn filter<C: Context, E: IntoValue<C>>(self, expr: E) -> DeleteWhere<'a, E::Expression, C> {
        DeleteWhere {
            table: self,
            expr: expr.into_expression(),
            _c: PhantomData,
        }
    }
}

impl<'a, C: Context> Statement<C> for Delete<'a> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        write!(ctx, "DELETE FROM {}", self.table)?;
        Ok(())
    }
}

pub struct DeleteWhere<'a, E, C> {
    table: Delete<'a>,
    expr: E,
    _c: PhantomData<C>,
}

impl<'a, E, C: Context> Statement<C> for DeleteWhere<'a, E, C>
where
    E: Expression<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.table.build(ctx)?;
        write!(ctx, " WHERE ")?;
        self.expr.build(ctx)?;
        Ok(())
    }
}

pub fn delete<'a>(table: impl Into<Cow<'a, str>>) -> Delete<'a> {
    Delete::new(table)
}
