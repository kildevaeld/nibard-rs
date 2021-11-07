use super::{BinaryExpression, BinaryOperator, Expression, LimitedSelect, Select};
use std::fmt::Write;

use crate::{Context, Error, Statement};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct SelectFilter<S, E>(pub S, pub E);

impl<S, E> SelectFilter<S, E> {
    pub fn new(sel: S, expr: E) -> SelectFilter<S, E> {
        SelectFilter(sel, expr)
    }
}

// impl<S, E, C: Context> SelectFilter<S, E>
// where
//     E: Expression<C>,
//     S: Select<C>,
// {
//     pub fn and<T>(self, expr: T) -> SelectFilter<S, BinaryExpression<E, T>>
//     where
//         T: Expression<C>,
//     {
//         SelectFilter(
//             self.0,
//             BinaryExpression {
//                 left: self.1,
//                 right: expr,
//                 operator: BinaryOperator::And,
//             },
//         )
//     }

//     pub fn or<T>(self, expr: T) -> SelectFilter<S, BinaryExpression<E, T>>
//     where
//         T: Expression<C>,
//     {
//         SelectFilter(
//             self.0,
//             BinaryExpression {
//                 left: self.1,
//                 right: expr,
//                 operator: BinaryOperator::Or,
//             },
//         )
//     }
// }

impl<S, E, C: Context> Select<C> for SelectFilter<S, E>
where
    S: Select<C>,
    E: Expression<C>,
{
    type Target = S::Target;
    type Selection = S::Selection;
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.0.build(ctx)?;
        ctx.write_str(" WHERE ")?;
        self.1.build(ctx)?;
        Ok(())
    }
}

impl<S, E, C: Context> Statement<C> for SelectFilter<S, E>
where
    S: Select<C>,
    E: Expression<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.0.build(ctx)?;
        ctx.write_str(" WHERE ")?;
        self.1.build(ctx)?;
        Ok(())
    }
}

// impl<S, E> Select for Filter<S, E>
// where
//     S: Select,
//     E: Expression,
// {
//     type Target = S::Target;
//     type Selection = S::Selection;
// }

// impl<S, E> SelectFilter for Filter<S, E>
// where
//     S: Select,
//     E: Expression,
// {
//     type Expression = E;
// }
