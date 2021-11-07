use super::{BinaryExpression, BinaryOperator, Expression, IntoValue, Joinable, Selection, Target};
use crate::{Context, Error, Statement};
use std::marker::PhantomData;

pub trait Select<C: Context> {
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<'a, C: Context> Select<C> for Box<dyn Select<C> + 'a> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

impl<'a, C: Context> JoinSelect<C> for Box<dyn Select<C> + 'a> {}

impl<'a, C: Context> FilterSelect<C> for Box<dyn Select<C> + 'a> {}

pub trait SelectExt<C: Context>: Select<C> + Sized {
    fn expr(self) -> SelectExpr<Self, C> {
        SelectExpr::new(self)
    }
}

impl<T, C: Context> SelectExt<C> for T where T: Select<C> {}

mod private {
    pub trait Sealed {}
}

pub trait LimitedSelect<C: Context>: Select<C> + Sized {
    fn limit(self, limit: impl Into<Option<u64>>) -> LimitedSel<Self, C> {
        LimitedSel::new(self).limit(limit)
    }

    fn offset(self, offset: impl Into<Option<u64>>) -> LimitedSel<Self, C> {
        LimitedSel::new(self).offset(offset)
    }
}

pub trait FilterSelect<C: Context>: Select<C> + Sized {
    fn filter<E: Expression<C>>(self, expr: E) -> FilterSel<Self, E> {
        FilterSel::new(self, expr)
    }
}

pub trait JoinSelect<C: Context>: Select<C> + Sized {
    fn join<J: Joinable<C>>(self, join: J) -> JoinSel<Self, J, C> {
        JoinSel::new(self, join)
    }
}

// Sel

#[derive(Clone, Debug)]
pub struct Sel<T, S> {
    target: T,
    selection: S,
}

impl<T, S> Sel<T, S> {
    pub fn new(target: T, selection: S) -> Sel<T, S> {
        Sel { target, selection }
    }
}

impl<T, S, C: Context> Select<C> for Sel<T, S>
where
    T: Target<C>,
    S: Selection<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str("SELECT ")?;
        self.selection.build(ctx)?;
        ctx.write_str(" FROM ")?;
        self.target.build(ctx)?;
        Ok(())
    }
}

impl<T, S, C: Context> Statement<C> for Sel<T, S>
where
    T: Target<C>,
    S: Selection<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <Sel<T, S> as Select<C>>::build(self, ctx)?;
        Ok(())
    }
}

impl<T, S, C: Context> LimitedSelect<C> for Sel<T, S>
where
    T: Target<C>,
    S: Selection<C>,
{
}

impl<T, S, C: Context> FilterSelect<C> for Sel<T, S>
where
    T: Target<C>,
    S: Selection<C>,
{
}

impl<T, S, C: Context> JoinSelect<C> for Sel<T, S>
where
    T: Target<C>,
    S: Selection<C>,
{
}

// Selelect offset limit

#[derive(Clone, Debug)]
pub struct LimitedSel<S, C: Context>
where
    S: Select<C>,
{
    select: S,
    limit: Option<u64>,
    offset: Option<u64>,
    _c: PhantomData<C>,
}

impl<S, C: Context> LimitedSel<S, C>
where
    S: Select<C>,
{
    pub fn new(select: S) -> LimitedSel<S, C> {
        LimitedSel {
            select,
            limit: None,
            offset: None,
            _c: PhantomData,
        }
    }

    pub fn limit(mut self, limit: impl Into<Option<u64>>) -> Self {
        self.limit = limit.into();
        self
    }

    pub fn offset(mut self, offset: impl Into<Option<u64>>) -> Self {
        self.offset = offset.into();
        self
    }
}

impl<S: Select<C>, C: Context> Select<C> for LimitedSel<S, C> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.select.build(ctx)?;

        if let Some(limit) = self.limit {
            write!(ctx, " LIMIT {}", limit)?;
        }

        if let Some(offset) = self.offset {
            write!(ctx, " OFFSET {}", offset)?;
        }

        Ok(())
    }
}

impl<S, C: Context> Statement<C> for LimitedSel<S, C>
where
    S: Select<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <LimitedSel<S, C> as Select<C>>::build(self, ctx)?;
        Ok(())
    }
}

// Selct join
#[derive(Clone, Debug)]
pub struct JoinSel<S, J, C: Context>
where
    S: Select<C>,
{
    select: S,
    join: J,
    _c: PhantomData<C>,
}

impl<'a, S, J, C: Context + 'a> JoinSel<S, J, C>
where
    S: Select<C> + 'a,
    J: Joinable<C> + 'a,
{
    pub fn new(select: S, join: J) -> JoinSel<S, J, C> {
        JoinSel {
            select,
            join,
            _c: PhantomData,
        }
    }

    pub fn boxed(self) -> Box<dyn Select<C> + 'a> {
        Box::new(self)
    }
}

impl<S, J, C: Context> Select<C> for JoinSel<S, J, C>
where
    S: Select<C>,
    J: Joinable<C>,
{
    // type Target = S::Target;
    // type Selection = S::Selection;
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.select.build(ctx)?;
        ctx.write_str(" ")?;
        self.join.build(ctx)?;
        Ok(())
    }
}

impl<S, J, C: Context> Statement<C> for JoinSel<S, J, C>
where
    S: Select<C>,
    J: Joinable<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <JoinSel<S, J, C> as Select<C>>::build(self, ctx)?;
        Ok(())
    }
}

impl<S, J, C: Context> LimitedSelect<C> for JoinSel<S, J, C>
where
    S: Select<C>,
    J: Joinable<C>,
{
}

impl<S, J, C: Context> FilterSelect<C> for JoinSel<S, J, C>
where
    S: Select<C>,
    J: Joinable<C>,
{
}

#[derive(Clone, Debug)]
pub struct FilterSel<S, E> {
    select: S,
    expr: E,
}

impl<S, E> FilterSel<S, E> {
    pub fn new(select: S, expr: E) -> FilterSel<S, E> {
        FilterSel { select, expr }
    }

    pub fn and<E1: Expression<C>, C: Context>(
        self,
        e: E1,
    ) -> FilterSel<S, BinaryExpression<E, E1, C>> {
        FilterSel {
            select: self.select,
            expr: BinaryExpression::new(self.expr, e, BinaryOperator::And),
        }
    }

    pub fn or<E1: Expression<C>, C: Context>(
        self,
        e: E1,
    ) -> FilterSel<S, BinaryExpression<E, E1, C>> {
        FilterSel {
            select: self.select,
            expr: BinaryExpression::new(self.expr, e, BinaryOperator::Or),
        }
    }
}

impl<S, E, C: Context> Select<C> for FilterSel<S, E>
where
    S: Select<C>,
    E: Expression<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.select.build(ctx)?;
        ctx.write_str(" WHERE ")?;
        self.expr.build(ctx)?;
        Ok(())
    }
}

impl<S, E, C: Context> Statement<C> for FilterSel<S, E>
where
    S: Select<C>,
    E: Expression<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <FilterSel<S, E> as Select<C>>::build(self, ctx)?;
        Ok(())
    }
}

impl<S, E, C: Context> LimitedSelect<C> for FilterSel<S, E>
where
    S: Select<C>,
    E: Expression<C>,
{
}

pub struct SelectExpr<S, C: Context>
where
    S: Select<C>,
{
    select: S,
    _c: PhantomData<C>,
}

impl<S, C: Context> SelectExpr<S, C>
where
    S: Select<C>,
{
    pub fn new(select: S) -> SelectExpr<S, C> {
        SelectExpr {
            select,
            _c: PhantomData,
        }
    }
}

impl<S, C: Context> Expression<C> for SelectExpr<S, C>
where
    S: Select<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_char('(')?;
        self.select.build(ctx)?;
        ctx.write_char(')')?;

        Ok(())
    }
}

impl<S, C: Context> IntoValue<C> for SelectExpr<S, C>
where
    S: Select<C>,
{
    type Expression = SelectExpr<S, C>;
    fn into_expression(self) -> Self::Expression {
        self
    }
}
