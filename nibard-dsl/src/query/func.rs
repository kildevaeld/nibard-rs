use super::{Column, Selection};
use crate::{Context, Error};

#[derive(Clone, Debug)]
pub enum FuncKind<C> {
    CountAll,
    Count(C),
}

#[derive(Clone, Debug)]
pub struct Func<C> {
    kind: FuncKind<C>,
}

impl Func<()> {
    pub fn count_all() -> Func<&'static str> {
        Func {
            kind: FuncKind::CountAll,
        }
    }

    pub fn count<C>(col: C) -> Func<C> {
        Func {
            kind: FuncKind::Count(col),
        }
    }
}

impl<Col: Column<C>, C: Context> Selection<C> for Func<Col> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        match &self.kind {
            FuncKind::CountAll => write!(ctx, "COUNT(*)")?,
            FuncKind::Count(col) => {
                ctx.write_char('(')?;
                <Col as Column<C>>::build(col, ctx)?; // col.build(ctx)?;
                ctx.write_char(')')?;
            }
        }
        Ok(())
    }
}

impl<Col: Column<C>, C: Context> Column<C> for Func<Col> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        match &self.kind {
            FuncKind::CountAll => write!(ctx, "COUNT(*)")?,
            FuncKind::Count(col) => {
                ctx.write_str("COUNT(")?;
                <Col as Column<C>>::build(col, ctx)?; // col.build(ctx)?;
                ctx.write_char(')')?;
            }
        }
        Ok(())
    }
}
