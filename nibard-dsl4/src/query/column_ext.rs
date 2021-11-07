use super::{BinaryExpression, BinaryOperator, Column, Expression, IntoExpression};
use crate::{Context, Error};
use std::borrow::Cow;
use std::fmt::Write;

pub trait ColExt<C: Context>: Column<C> + Sized {
    fn eql<'a, E: IntoExpression<C>>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Eq,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn lt<'a, E: IntoExpression<C>>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Lt,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn lte<'a, E: IntoExpression<C>>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Lte,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn gt<'a, E: IntoExpression<C>>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Gt,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn gte<'a, E: IntoExpression<C>>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Gte,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn neq<'a, E: IntoExpression<C>>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::NotEq,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn like<'a, E: IntoExpression<C>>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Like,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn contains<'a, E: IntoExpression<C>>(
        self,
        e: E,
    ) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::In,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn alias<'a>(self, name: impl Into<Cow<'a, str>>) -> ColAlias<'a, Self> {
        ColAlias {
            col: self,
            name: name.into(),
        }
    }
}

impl<C, CTX: Context> ColExt<CTX> for C where C: Column<CTX> {}

#[derive(Clone, Hash)]
pub struct ColExp<C> {
    col: C,
}

impl<C> ColExp<C> {
    pub fn new(col: C) -> ColExp<C> {
        ColExp { col }
    }
}

impl<C, CTX: Context> Expression<CTX> for ColExp<C>
where
    C: Column<CTX>,
{
    fn build(&self, ctx: &mut CTX) -> Result<(), Error> {
        Ok(self.col.build(ctx)?)
    }
}

#[derive(Clone, Debug)]
pub struct ColAlias<'a, C> {
    col: C,
    name: Cow<'a, str>,
}

impl<'a, C> ColAlias<'a, C> {
    pub fn col(&self) -> &C {
        &self.col
    }
}

impl<'a, C, CTX: Context> Column<CTX> for ColAlias<'a, C>
where
    C: Column<CTX>,
{
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn build(&self, ctx: &mut CTX) -> Result<(), Error> {
        self.col.build(ctx)?;
        write!(ctx, " AS {}", self.name)?;
        Ok(())
    }
}