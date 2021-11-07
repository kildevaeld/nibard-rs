// use super::Expression;
use crate::{Context, Error};
use std::marker::PhantomData;

pub trait Expression<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

pub trait ExpressionExt<'a, C: Context>: Expression<C> + Sized {
    fn boxed(self) -> Box<dyn Expression<C> + 'a>
    where
        Self: 'a,
    {
        Box::new(self)
    }
}

impl<'a, E: Expression<C>, C: Context> ExpressionExt<'a, C> for E {}

impl<'a, C: Context> Expression<C> for Box<dyn Expression<C> + 'a> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

pub struct BinaryExpression<L, R, C: Context> {
    pub(crate) operator: BinaryOperator,
    pub(crate) left: L,
    pub(crate) right: R,
    _c: PhantomData<C>,
}

impl<'a, L, R, C: Context> BinaryExpression<L, R, C> {
    pub fn new(left: L, right: R, operator: BinaryOperator) -> Self {
        BinaryExpression {
            left,
            right,
            operator,
            _c: PhantomData,
        }
    }

    pub fn and<E: Expression<C>>(self, expr: E) -> BinaryExpression<Self, E, C> {
        BinaryExpression::new(self, expr, BinaryOperator::And)
    }

    pub fn or<E: Expression<C>>(self, expr: E) -> BinaryExpression<Self, E, C> {
        BinaryExpression::new(self, expr, BinaryOperator::Or)
    }
}

pub enum BinaryOperator {
    Eq,
    Lt,
    Lte,
    Gt,
    Gte,
    NotEq,
    And,
    Or,
    Like,
    In,
}

impl BinaryOperator {
    fn build<C: Context>(&self, ctx: &mut C) -> Result<(), Error> {
        match self {
            Self::Eq => ctx.write_str("="),
            Self::Lt => ctx.write_str("<"),
            Self::Lte => ctx.write_str("<="),
            Self::Gt => ctx.write_str(">"),
            Self::Gte => ctx.write_str(">="),
            Self::NotEq => ctx.write_str("!="),
            Self::And => ctx.write_str("AND"),
            Self::Or => ctx.write_str("OR"),
            Self::Like => ctx.write_str("LIKE"),
            Self::In => ctx.write_str("IN"),
        }?;
        Ok(())
    }
}

impl<L, R, C: Context> Expression<C> for BinaryExpression<L, R, C>
where
    L: Expression<C>,
    R: Expression<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.left.build(ctx)?;
        ctx.write_str(" ")?;
        self.operator.build(ctx)?;
        ctx.write_str(" ")?;
        self.right.build(ctx)?;
        Ok(())
    }
}
