use crate::{Context, Error};

pub trait Expression {
    fn build(&self, ctx: &mut Context) -> Result<(), Error>;
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
