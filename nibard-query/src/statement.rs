use super::{Context, Error};

pub trait Statement {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error>;
}
