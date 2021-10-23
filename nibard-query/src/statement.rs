use super::{Context, Error};

pub trait Statement {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error>;
}

impl Statement for Box<dyn Statement> {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        (**self).build(ctx)
    }
}
