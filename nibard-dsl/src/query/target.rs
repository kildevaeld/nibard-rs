use super::{Sel, Select, Selection, Table};
use crate::{Context, Error};
use std::fmt::Write;

pub trait Target {
    fn build(&self, ctx: &mut Context) -> Result<(), Error>;
}

pub trait TargetExt<S: Selection>: Target + Sized {
    type Select: Select<Target = Self>;
    fn select(self, selection: S) -> Self::Select;
}

impl<T, S> TargetExt<S> for T
where
    T: Target,
    S: Selection,
{
    type Select = Sel<Self, S>;
    fn select(self, selection: S) -> Self::Select {
        Sel(self, selection)
    }
}

macro_rules! selection {
    ($n: tt => $first: ident) => {
        impl<$first: Table> Target for $first {
            fn build(&self, ctx: &mut Context) -> Result<(), Error> {
                <$first as Table>::build(self, ctx)?;
                Ok(())
            }
        }

    };
    ($n1: tt => $type1:ident, $( $n: tt => $type:ident  ),*) => {
        selection!($($n => $type),*);

        impl<$type1: Table, $( $type: Table ),*> Target for ($type1, $($type),*)  {
            fn build(&self, ctx: &mut Context) -> Result<(), Error> {
                self.$n1.build(ctx)?;
                $(
                    ctx.write_str(", ")?;
                    self.$n.build(ctx)?;
                )*
                Ok(())
            }
        }
    };
}

selection!(
    16 => C16,
    15 => C15,
    14 => C14,
    13 => C13,
    12 => C12,
    11 => C11,
    10 => C10,
    9 => C9,
    8 => C8,
    7 => C7,
    6 => C6,
    5 => C5,
    4 => C4,
    3 => C3,
    2 => C2,
    1 => C1,
    0 => C0
);
