use super::{Context, Error};

pub trait Statement<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}
