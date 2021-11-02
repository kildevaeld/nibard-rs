use super::{BinaryExpression, BinaryOperator, Expression, LimitedSelect, Select};
use std::fmt::Write;

use crate::{Context, Error, Statement};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct SelectFilter<S, E>(pub S, pub E);

impl<S, E> SelectFilter<S, E>
where
    E: Expression,
    S: Select,
{
    pub fn and<T>(self, expr: T) -> SelectFilter<S, BinaryExpression<E, T>>
    where
        T: Expression,
    {
        SelectFilter(
            self.0,
            BinaryExpression {
                left: self.1,
                right: expr,
                operator: BinaryOperator::And,
            },
        )
    }

    pub fn or<T>(self, expr: T) -> SelectFilter<S, BinaryExpression<E, T>>
    where
        T: Expression,
    {
        SelectFilter(
            self.0,
            BinaryExpression {
                left: self.1,
                right: expr,
                operator: BinaryOperator::And,
            },
        )
    }

    // pub fn offset(self, offset: u64) -> LimitedSelect<Self>
    // where
    //     Self: Sized,
    // {
    //     LimitedSelect {
    //         sel: self,
    //         offset: Some(offset),
    //         limit: None,
    //     }
    // }

    // pub fn limit(self, limit: u64) -> LimitedSelect<Self>
    // where
    //     Self: Sized,
    // {
    //     LimitedSelect {
    //         sel: self,
    //         offset: None,
    //         limit: Some(limit),
    //     }
    // }
}

impl<S, E> Select for SelectFilter<S, E>
where
    S: Select,
    E: Expression,
{
    type Target = S::Target;
    type Selection = S::Selection;
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        self.0.build(ctx)?;
        ctx.write_str(" WHERE ")?;
        self.1.build(ctx)?;
        Ok(())
    }
}

impl<S, E> Statement for SelectFilter<S, E>
where
    S: Select,
    E: Expression,
{
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
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
