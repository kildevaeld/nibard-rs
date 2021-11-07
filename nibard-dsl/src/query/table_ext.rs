use super::{
    Column, FilterSelect, JoinSelect, LimitedSelect, Sel, Select, Selection, Table, Target,
};
use crate::{Context, Error, Statement};
use std::marker::PhantomData;

pub trait TargetExt<C: Context>: Target<C> + Sized {
    fn select<S: Selection<C>>(self, selection: S) -> TargetSelect<Self, S, C> {
        TargetSelect {
            select: Sel::new(self, selection),
            _c: PhantomData,
        }
    }
}

impl<T, C: Context> TargetExt<C> for T where T: Target<C> {}

pub struct TargetSelect<T, S, C> {
    select: Sel<T, S>,
    _c: PhantomData<C>,
}

impl<T, S, C: Context> Select<C> for TargetSelect<T, S, C>
where
    T: Target<C>,
    S: Selection<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <Sel<T, S> as Select<C>>::build(&self.select, ctx)?;
        Ok(())
    }
}

impl<T, S, C: Context> LimitedSelect<C> for TargetSelect<T, S, C>
where
    T: Target<C>,
    S: Selection<C>,
{
}

impl<T, S, C: Context> FilterSelect<C> for TargetSelect<T, S, C>
where
    T: Target<C>,
    S: Selection<C>,
{
}

impl<T, S, C: Context> JoinSelect<C> for TargetSelect<T, S, C>
where
    T: Target<C>,
    S: Selection<C>,
{
}

impl<T, S, C: Context> Statement<C> for TargetSelect<T, S, C>
where
    T: Target<C>,
    S: Selection<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <Sel<T, S> as Statement<C>>::build(&self.select, ctx)?;
        Ok(())
    }
}

pub trait TableExt<C: Context>: Table<C> + Sized {
    fn col<Col: Column<C>>(self, col: Col) -> TableCol<Self, Col, C> {
        TableCol::new(self, col)
    }
}

impl<T, C: Context> TableExt<C> for T where T: Table<C> {}

pub struct TableCol<T, C, CTX: Context> {
    column: C,
    table: T,
    _c: PhantomData<CTX>,
}

impl<T, C, CTX: Context> TableCol<T, C, CTX> {
    pub fn new(table: T, column: C) -> TableCol<T, C, CTX> {
        TableCol {
            column,
            table,
            _c: PhantomData,
        }
    }
}

impl<T, C, CTX: Context> Selection<CTX> for TableCol<T, C, CTX> {
    fn build(&self, ctx: &mut CTX) -> Result<(), Error> {
        Ok(())
    }
}

impl<T, C, CTX: Context> Column<CTX> for TableCol<T, C, CTX>
where
    T: Table<CTX>,
    C: Column<CTX>,
{
    fn build(&self, ctx: &mut CTX) -> Result<(), Error> {
        <T as Table<CTX>>::build(&self.table, ctx)?;
        ctx.write_char('.')?;
        <C as Column<CTX>>::build(&self.column, ctx)?;
        Ok(())
    }
}
