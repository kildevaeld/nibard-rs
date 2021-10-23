use super::{Column, Context, Error, Target};
use std::fmt::Write;

pub trait Selection {
    fn build(&self, ctx: &mut Context) -> Result<(), Error>;
}

pub trait SelectionExt: Selection {
    fn select_from<T>(self, target: T)
    where
        Self: Sized,
        T: Target;
}

macro_rules! selection {
    ($n: tt => $first: ident) => {
        impl<$first: Column> Selection for $first {
            #[inline]
            fn build(&self, ctx: &mut Context) -> Result<(),$crate::Error> {
                <$first as Column>::build(self, ctx)?;
                Ok(())
            }
        }

    };
    ($n1: tt => $type1:ident, $( $n: tt => $type:ident  ),*) => {
        selection!($($n => $type),*);

        impl<$type1: Column, $( $type: Column ),*> Selection for ($type1, $($type),*) {

            #[inline]
            fn build(&self, ctx: &mut Context) -> Result<(),$crate::Error> {
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

impl<V> Selection for Vec<V>
where
    V: Column,
{
    #[inline]
    fn build(&self, ctx: &mut Context) -> Result<(), crate::Error> {
        for (idx, col) in self.iter().enumerate() {
            if idx != 0 {
                ctx.write_char(',')?;
            }
            col.build(ctx)?;
        }
        Ok(())
    }
}
