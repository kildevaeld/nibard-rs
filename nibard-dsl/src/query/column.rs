use crate::{Context, Error};
use std::fmt::Write;

pub trait Column {
    fn name(&self) -> &str;
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        Ok(ctx.write_str(self.name())?)
    }
}

impl<'a, T> Column for &'a T
where
    T: Column,
{
    fn name(&self) -> &str {
        (&**self).name()
    }
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

impl<'a> Column for &'a str {
    fn name(&self) -> &str {
        self
    }
}

impl Column for String {
    fn name(&self) -> &str {
        self
    }
}

pub fn col<C: Column>(col: C) -> impl Column {
    col
}
