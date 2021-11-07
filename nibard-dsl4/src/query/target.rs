use super::{Sel, Select, Selection, Table};
use crate::{Context, Error};
use std::fmt::Write;

pub trait Target<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<'a, C: Context> Target<C> for Box<dyn Target<C> + 'a> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

pub trait TargetExt<S: Selection<C>, C: Context>: Target<C> + Sized {
    type Select: Select<C, Target = Self>;
    fn select(self, selection: S) -> Self::Select;
}

impl<T, S, C: Context> TargetExt<S, C> for T
where
    T: Target<C>,
    S: Selection<C>,
{
    type Select = Sel<Self, S>;
    fn select(self, selection: S) -> Self::Select {
        Sel {
            target: self,
            selection,
        }
    }
}

macro_rules! selection {
    ($n: tt => $first: ident) => {
        impl<C: Context, $first: Table<C>> Target<C> for ($first,) {
            fn build(&self, ctx: &mut C) -> Result<(), Error> {
                <$first as Table<C>>::build(&self.0, ctx)?;
                Ok(())
            }
        }

    };
    ($n1: tt => $type1:ident, $( $n: tt => $type:ident  ),*) => {
        selection!($($n => $type),*);

        impl<C: Context,$type1: Table<C>, $( $type: Table<C> ),*> Target<C> for ($type1, $($type),*)  {
            fn build(&self, ctx: &mut C) -> Result<(), Error> {
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
