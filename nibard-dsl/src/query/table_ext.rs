use super::{
    Alias, Column, FilterSelect, JoinSelect, LimitedSelect, Sel, Select, Selection, Table, Target,
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

#[derive(Debug)]
pub struct TargetSelect<T, S, C> {
    select: Sel<T, S>,
    _c: PhantomData<C>,
}

impl<'a, T, S, C: Context> TargetSelect<T, S, C>
where
    T: Target<C>,
    S: Selection<C>,
{
    pub fn boxed(self) -> Box<dyn Select<C> + 'a>
    where
        Self: 'a,
    {
        Box::new(self)
    }
}

impl<T: Clone, S: Clone, C> Clone for TargetSelect<T, S, C> {
    fn clone(&self) -> Self {
        TargetSelect {
            select: self.select.clone(),
            _c: PhantomData,
        }
    }
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

    fn table_alias<A: Alias<C>>(self, alias: A) -> TableAlias<Self, A, C> {
        TableAlias::new(self, alias)
    }
}

impl<T, C: Context> TableExt<C> for T where T: Table<C> {}

#[derive(Debug)]
pub struct TableAlias<T, A, C> {
    table: T,
    alias: A,
    _c: PhantomData<C>,
}

impl<T: Clone, A: Clone, C> Clone for TableAlias<T, A, C> {
    fn clone(&self) -> Self {
        TableAlias {
            table: self.table.clone(),
            alias: self.alias.clone(),
            _c: PhantomData,
        }
    }
}

impl<T, A, C> TableAlias<T, A, C> {
    pub fn new(table: T, alias: A) -> TableAlias<T, A, C> {
        TableAlias {
            table,
            alias,
            _c: PhantomData,
        }
    }
}

impl<T, A, C: Context> Target<C> for TableAlias<T, A, C>
where
    T: Target<C>,
    A: Alias<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.table.build(ctx)?;
        write!(ctx, " AS ")?;
        self.alias.build(ctx)?;
        Ok(())
    }
}

impl<T, A, C: Context> Table<C> for TableAlias<T, A, C>
where
    T: Table<C>,
    A: Alias<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <T as Table<C>>::build(&self.table, ctx)?;
        Ok(())
    }
}

// Table column

#[derive(Debug)]
pub struct TableCol<T, C, CTX: Context> {
    column: C,
    table: T,
    _c: PhantomData<CTX>,
}

impl<T: Clone, C: Clone, CTX: Context> Clone for TableCol<T, C, CTX> {
    fn clone(&self) -> Self {
        TableCol {
            column: self.column.clone(),
            table: self.table.clone(),
            _c: PhantomData,
        }
    }
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

impl<T, C, CTX: Context> Selection<CTX> for TableCol<T, C, CTX>
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
