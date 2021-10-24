use super::Expression;
use crate::{Context, Error};
use std::fmt::Write;

pub struct GroupExpression<E>(pub E);

impl<E: Expression> Expression for GroupExpression<E> {
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        write!(ctx, "(")?;
        self.0.build(ctx)?;
        write!(ctx, ")")?;
        Ok(())
    }
}
