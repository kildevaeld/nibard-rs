use crate::{Context, Error};
use std::fmt::Write;

pub trait Column<C: Context> {
    fn name(&self) -> &str;
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        Ok(ctx.write_str(self.name())?)
    }
}

impl<'a, T, C: Context> Column<C> for &'a T
where
    T: Column<C>,
{
    fn name(&self) -> &str {
        (&**self).name()
    }
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

impl<'a, C: Context> Column<C> for &'a str {
    fn name(&self) -> &str {
        self
    }
}

impl<C: Context> Column<C> for String {
    fn name(&self) -> &str {
        self
    }
}

pub fn col<C: Column<CTX>, CTX: Context>(col: C) -> impl Column<CTX> {
    col
}
