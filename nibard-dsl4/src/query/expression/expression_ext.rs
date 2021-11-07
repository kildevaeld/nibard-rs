use super::{BinaryExpression, BinaryOperator, Expression, GroupExpression, IntoExpression};
use crate::Context;

pub trait ExpressionExt<C: Context>: Expression<C> + Sized {
    fn and<E>(self, e: E) -> BinaryExpression<Self, E> {
        BinaryExpression {
            operator: BinaryOperator::And,
            left: self,
            right: e,
        }
    }

    fn or<E>(self, e: E) -> BinaryExpression<Self, E> {
        BinaryExpression {
            operator: BinaryOperator::Or,
            left: self,
            right: e,
        }
    }

    fn and_group<E: IntoExpression<C>>(self, e: E) -> BinaryExpression<Self, GroupExpression<E>> {
        BinaryExpression {
            operator: BinaryOperator::And,
            left: self,
            right: GroupExpression(e),
        }
    }

    fn or_group<E>(self, e: E) -> BinaryExpression<Self, GroupExpression<E>> {
        BinaryExpression {
            operator: BinaryOperator::Or,
            left: self,
            right: GroupExpression(e),
        }
    }
}

impl<'a, E, C: Context> ExpressionExt<C> for E where E: Expression<C> {}
