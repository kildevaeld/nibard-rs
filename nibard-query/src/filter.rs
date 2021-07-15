use super::{BinaryExpression, BinaryOperator, Context, Error, Expression, Select, Statement};
use std::fmt::Write;

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
