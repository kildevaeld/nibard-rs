use crate::{Context, Error};

pub trait Alias<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<'a, A, C: Context> Alias<C> for &'a A
where
    A: Alias<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

impl<'a, C: Context> Alias<C> for &'a str {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        Ok(ctx.write_str(self)?)
    }
}

impl<C: Context> Alias<C> for String {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        Ok(ctx.write_str(self)?)
    }
}

// Target

pub trait Target<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<'a, T, C: Context> Target<C> for &'a T
where
    T: Table<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <T as Target<C>>::build(&**self, ctx)
    }
}

impl<'a, C: Context> Target<C> for &'a str {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str(self)?;
        Ok(())
    }
}

impl<'a, C: Context> Table<C> for &'a str {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str(self)?;
        Ok(())
    }
}

impl<C: Context> Target<C> for String {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str(self)?;
        Ok(())
    }
}

impl<C: Context> Table<C> for String {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str(self)?;
        Ok(())
    }
}

macro_rules! selection {
    ($n: tt => $first: ident) => {
        impl<C: Context, $first: Target<C>> Target<C> for ($first,) {
            fn build(&self, ctx: &mut C) -> Result<(), Error> {
                <$first as Target<C>>::build(&self.0, ctx)?;
                Ok(())
            }
        }

    };
    ($n1: tt => $type1:ident, $( $n: tt => $type:ident  ),*) => {
        selection!($($n => $type),*);

        impl<C: Context,$type1: Target<C>, $( $type: Target<C> ),*> Target<C> for ($type1, $($type),*)  {
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

// Table

pub trait Table<C: Context>: Target<C> {
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

// Selection

pub trait Selection<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<'a, T, C: Context> Selection<C> for &'a T
where
    T: Selection<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

impl<'a, C: Context> Selection<C> for &'a str {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str(self)?;
        Ok(())
    }
}

impl<'a, C: Context> Column<C> for &'a str {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str(self)?;
        Ok(())
    }
}

impl<C: Context> Selection<C> for String {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str(self)?;
        Ok(())
    }
}

impl<C: Context> Column<C> for String {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str(self)?;
        Ok(())
    }
}

macro_rules! selection {
    ($n: tt => $first: ident) => {
        impl<C: Context,$first: Selection<C>> Selection<C> for ($first,) {
            #[inline]
            fn build(&self, ctx: &mut C) -> Result<(),$crate::Error> {
                <$first as Selection<C>>::build(&self.0, ctx)?;
                Ok(())
            }
        }

    };
    ($n1: tt => $type1:ident, $( $n: tt => $type:ident  ),*) => {
        selection!($($n => $type),*);

        impl<C: Context,$type1: Selection<C>, $( $type: Selection<C> ),*> Selection<C> for ($type1, $($type),*) {

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
    V: Selection<C>,
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

// Column

pub trait Column<C: Context>: Selection<C> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<'a, T, C: Context> Column<C> for &'a T
where
    T: Column<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <T as Column<C>>::build(&**self, ctx)
        // (&**self).build(ctx)
    }
}

// pub trait ColumnList<C: Context> {
//     fn build(&self, ctx: &mut C) -> Result<(), Error>;
// }

// macro_rules! columns {
//     ($n: tt => $first: ident) => {
//         impl<C: Context, $first: Column<C>> ColumnList<C> for ($first,) {
//             fn build(&self, ctx: &mut C) -> Result<(), Error> {
//                 <$first as Column<C>>::build(&self.0, ctx)?;
//                 Ok(())
//             }
//         }

//     };
//     ($n1: tt => $type1:ident, $( $n: tt => $type:ident  ),*) => {
//         columns!($($n => $type),*);

//         impl<C: Context,$type1: Column<C>, $( $type: Column<C> ),*> ColumnList<C> for ($type1, $($type),*)  {
//             fn build(&self, ctx: &mut C) -> Result<(), Error> {
//                 // <$type1 as Column<C>>::build(self.$n1, ctx);
//                 let col: &dyn Column<C> = &self.$n1;
//                 col.build(ctx)?;
//                 $(
//                     ctx.write_str(", ")?;
//                     self.$n.build(ctx)?;
//                 )*
//                 Ok(())
//             }
//         }
//     };
// }

// columns!(
//     16 => C16,
//     15 => C15,
//     14 => C14,
//     13 => C13,
//     12 => C12,
//     11 => C11,
//     10 => C10,
//     9 => C9,
//     8 => C8,
//     7 => C7,
//     6 => C6,
//     5 => C5,
//     4 => C4,
//     3 => C3,
//     2 => C2,
//     1 => C1,
//     0 => C0
// );

#[derive(Clone, Debug)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B, C: Context> Selection<C> for Either<A, B>
where
    A: Selection<C>,
    B: Selection<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        match self {
            Either::Left(a) => a.build(ctx),
            Either::Right(b) => b.build(ctx),
        }
    }
}
