use crate::{Context, Error};

pub trait Statement<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<C: Context> Statement<C> for Box<dyn Statement<C> + Send> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (**self).build(ctx)
    }
}

impl<C: Context> Statement<C> for Box<dyn Statement<C> + Send + Sync> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (**self).build(ctx)
    }
}
