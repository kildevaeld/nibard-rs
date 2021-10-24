use super::{BinaryExpression, BinaryOperator, Expression, GroupExpression};

pub trait ExpressionExt: Expression + Sized {
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

    fn and_group<E>(self, e: E) -> BinaryExpression<Self, GroupExpression<E>> {
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

impl<'a, E> ExpressionExt for E where E: Expression {}
