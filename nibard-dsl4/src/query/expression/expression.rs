use crate::{Context, Error};

pub trait Expression<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<C: Context> Expression<C> for Box<dyn Expression<C>> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (**self).build(ctx)
    }
}

impl<C: Context> Expression<C> for Box<dyn Expression<C> + Send> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (**self).build(ctx)
    }
}

impl<C: Context> Expression<C> for Box<dyn Expression<C> + Send + Sync> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (**self).build(ctx)
    }
}

pub trait IntoExpression<C: Context> {
    type Expression: Expression<C>;
    fn into_expression(self) -> Self::Expression;
}

// impl<E, C: Context> IntoExpression<C> for E
// where
//     E: Expression<C>,
// {
//     type Expression = E;
//     fn into_expression(self) -> Self::Expression {
//         self
//     }
// }
