use crate::{Context, Error};

pub trait Expression {
    fn build(&self, ctx: &mut Context) -> Result<(), Error>;
}

impl Expression for Box<dyn Expression> {
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        (**self).build(ctx)
    }
}

impl Expression for Box<dyn Expression + Send> {
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        (**self).build(ctx)
    }
}

impl Expression for Box<dyn Expression + Send + Sync> {
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        (**self).build(ctx)
    }
}

pub trait IntoExpression {
    type Expression: Expression;
    fn into_expression(self) -> Self::Expression;
}

impl<'a, E> IntoExpression for E
where
    E: Expression,
{
    type Expression = E;
    fn into_expression(self) -> Self::Expression {
        self
    }
}
