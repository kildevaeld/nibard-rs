use super::{BinaryExpression, BinaryOperator, Column, Expression, IntoExpression};
use crate::{Context, Error};
use std::borrow::Cow;
use std::fmt::Write;

pub trait ColExt: Column + Sized {
    fn eql<'a, E: IntoExpression>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Eq,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn lt<'a, E: IntoExpression>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Lt,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn lte<'a, E: IntoExpression>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Lte,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn gt<'a, E: IntoExpression>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Gt,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn gte<'a, E: IntoExpression>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Gte,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn neq<'a, E: IntoExpression>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::NotEq,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn like<'a, E: IntoExpression>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
    where
        Self: 'a,
    {
        BinaryExpression {
            operator: BinaryOperator::Like,
            left: ColExp { col: self },
            right: e.into_expression(),
        }
    }

    fn contains<'a, E: IntoExpression>(self, e: E) -> BinaryExpression<ColExp<Self>, E::Expression>
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

impl<C> ColExt for C where C: Column {}

#[derive(Clone, Hash)]
pub struct ColExp<C> {
    col: C,
}

impl<C> ColExp<C> {
    pub fn new(col: C) -> ColExp<C> {
        ColExp { col }
    }
}

impl<C> Expression for ColExp<C>
where
    C: Column,
{
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        Ok(self.col.build(ctx)?)
    }
}

pub struct ColAlias<'a, C> {
    col: C,
    name: Cow<'a, str>,
}

impl<'a, C> Column for ColAlias<'a, C>
where
    C: Column,
{
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        self.col.build(ctx)?;
        write!(ctx, " AS {}", self.name)?;
        Ok(())
    }
}
