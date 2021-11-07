use crate::{Context, Error};

pub trait Alias<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
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

// Selection

pub trait Selection<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
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

// Column

pub trait Column<C: Context>: Selection<C> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}
