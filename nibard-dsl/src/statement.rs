use crate::{Context, Error};

pub trait Statement {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error>;
}

impl Statement for Box<dyn Statement + Send> {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        (**self).build(ctx)
    }
}

impl Statement for Box<dyn Statement + Send + Sync> {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        (**self).build(ctx)
    }
}
