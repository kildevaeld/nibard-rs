use super::{Context, Error};

pub trait Statement<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

pub trait Table<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<'a, T, C: Context> Table<C> for &'a T
where
    T: Table<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <T as Table<C>>::build(&**self, ctx)
    }
}
