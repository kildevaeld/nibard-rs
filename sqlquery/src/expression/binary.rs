use super::{Expression, IntoExpression};
use crate::{Context, Error};
use std::fmt::Write as _;

pub struct BinaryExpression<L, R> {
    pub(crate) operator: BinaryOperator,
    pub(crate) left: L,
    pub(crate) right: R,
}

impl<'a, L, R> BinaryExpression<L, R>
where
    L: Expression,
    R: Expression,
{
    pub fn new(left: L, right: R, operator: BinaryOperator) -> Self {
        BinaryExpression {
            left,
            right,
            operator,
        }
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
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
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

impl<L, R> Expression for BinaryExpression<L, R>
where
    L: Expression,
    R: Expression,
{
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        self.left.build(ctx)?;
        ctx.write_str(" ")?;
        self.operator.build(ctx)?;
        ctx.write_str(" ")?;
        self.right.build(ctx)?;
        Ok(())
    }
}
