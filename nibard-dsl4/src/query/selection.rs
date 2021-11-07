use super::{Column, Target};
use crate::{Context, Error};
use std::fmt::Write;

pub trait Selection<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<'a, C: Context> Selection<C> for Box<dyn Selection<C> + 'a> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

pub trait SelectionExt<C: Context>: Selection<C> {
    fn select_from<T>(self, target: T)
    where
        Self: Sized,
        T: Target<C>;
}

macro_rules! selection {
    ($n: tt => $first: ident) => {
        impl<C: Context,$first: Column<C>> Selection<C> for ($first,) {
            #[inline]
            fn build(&self, ctx: &mut C) -> Result<(),$crate::Error> {
                <$first as Column<C>>::build(&self.0, ctx)?;
                Ok(())
            }
        }

    };
    ($n1: tt => $type1:ident, $( $n: tt => $type:ident  ),*) => {
        selection!($($n => $type),*);

        impl<C: Context,$type1: Column<C>, $( $type: Column<C> ),*> Selection<C> for ($type1, $($type),*) {

            #[inline]
            fn build(&self, ctx: &mut C) -> Result<(),$crate::Error> {
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
    20 => C20,
    19 => C19,
    18 => C18,
    17 => C17,
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

impl<V, C: Context> Selection<C> for Vec<V>
where
    V: Column<C>,
{
    #[inline]
    fn build(&self, ctx: &mut C) -> Result<(), crate::Error> {
        for (idx, col) in self.iter().enumerate() {
            if idx != 0 {
                ctx.write_char(',')?;
            }
            col.build(ctx)?;
        }
        Ok(())
    }
}
