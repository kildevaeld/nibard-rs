use super::Expression;
use crate::{Context, Error};
use std::fmt::Write;

pub struct GroupExpression<E>(pub E);

impl<E: Expression<C>, C: Context> Expression<C> for GroupExpression<E> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        write!(ctx, "(")?;
        self.0.build(ctx)?;
        write!(ctx, ")")?;
        Ok(())
    }
}
