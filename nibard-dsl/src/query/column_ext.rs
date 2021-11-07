use super::{Alias, BinaryExpression, BinaryOperator, Column, Expression, Selection};
use crate::{Context, Error};
use std::marker::PhantomData;

pub trait IntoValue<C: Context> {
    type Expression: Expression<C>;
    fn into_expression(self) -> Self::Expression;
}

pub trait ColumnExt<C: Context>: Column<C> + Sized {
    fn eql<'a, E: IntoValue<C>>(self, e: E) -> BinaryExpression<ColExpr<Self, C>, E::Expression, C>
    where
        Self: 'a,
    {
        BinaryExpression::new(ColExpr::new(self), e.into_expression(), BinaryOperator::Eq)
    }

    fn neq<'a, E: IntoValue<C>>(self, e: E) -> BinaryExpression<ColExpr<Self, C>, E::Expression, C>
    where
        Self: 'a,
    {
        BinaryExpression::new(
            ColExpr::new(self),
            e.into_expression(),
            BinaryOperator::NotEq,
        )
    }

    fn lt<'a, E: IntoValue<C>>(self, e: E) -> BinaryExpression<ColExpr<Self, C>, E::Expression, C>
    where
        Self: 'a,
    {
        BinaryExpression::new(ColExpr::new(self), e.into_expression(), BinaryOperator::Lt)
    }

    fn lte<'a, E: IntoValue<C>>(self, e: E) -> BinaryExpression<ColExpr<Self, C>, E::Expression, C>
    where
        Self: 'a,
    {
        BinaryExpression::new(ColExpr::new(self), e.into_expression(), BinaryOperator::Lte)
    }

    fn gt<'a, E: IntoValue<C>>(self, e: E) -> BinaryExpression<ColExpr<Self, C>, E::Expression, C>
    where
        Self: 'a,
    {
        BinaryExpression::new(ColExpr::new(self), e.into_expression(), BinaryOperator::Gt)
    }

    fn gte<'a, E: IntoValue<C>>(self, e: E) -> BinaryExpression<ColExpr<Self, C>, E::Expression, C>
    where
        Self: 'a,
    {
        BinaryExpression::new(ColExpr::new(self), e.into_expression(), BinaryOperator::Gte)
    }

    fn like<'a, E: IntoValue<C>>(self, e: E) -> BinaryExpression<ColExpr<Self, C>, E::Expression, C>
    where
        Self: 'a,
    {
        BinaryExpression::new(
            ColExpr::new(self),
            e.into_expression(),
            BinaryOperator::Like,
        )
    }

    fn has<'a, E: IntoValue<C>>(self, e: E) -> BinaryExpression<ColExpr<Self, C>, E::Expression, C>
    where
        Self: 'a,
    {
        BinaryExpression::new(ColExpr::new(self), e.into_expression(), BinaryOperator::In)
    }

    fn column_alias<A: Alias<C>>(self, alias: A) -> ColAlias<Self, A, C> {
        ColAlias::new(self, alias)
    }

    fn expr(self) -> ColExpr<Self, C> {
        ColExpr::new(self)
    }
}

impl<C, CTX: Context> ColumnExt<CTX> for C where C: Column<CTX> {}

#[derive(Clone, Hash)]
pub struct ColExpr<C, CTX: Context> {
    col: C,
    _c: PhantomData<CTX>,
}

impl<C, CTX: Context> ColExpr<C, CTX> {
    pub fn new(col: C) -> ColExpr<C, CTX> {
        ColExpr {
            col,
            _c: PhantomData,
        }
    }
}

impl<C, CTX: Context> Expression<CTX> for ColExpr<C, CTX>
where
    C: Column<CTX>,
{
    fn build(&self, ctx: &mut CTX) -> Result<(), Error> {
        <C as Column<CTX>>::build(&self.col, ctx)?;
        Ok(())
    }
}

impl<C: Column<CTX>, CTX: Context> IntoValue<CTX> for ColExpr<C, CTX> {
    type Expression = ColExpr<C, CTX>;
    fn into_expression(self) -> Self::Expression {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ColAlias<Col, A, C> {
    col: Col,
    alias: A,
    _c: PhantomData<C>,
}

impl<Col, A, C> ColAlias<Col, A, C> {
    pub fn new(col: Col, alias: A) -> ColAlias<Col, A, C> {
        ColAlias {
            col,
            alias,
            _c: PhantomData,
        }
    }
    pub fn col(&self) -> &Col {
        &self.col
    }
}

impl<C, A, CTX: Context> Selection<CTX> for ColAlias<C, A, CTX>
where
    C: Column<CTX>,
    A: Alias<CTX>,
{
    fn build(&self, ctx: &mut CTX) -> Result<(), Error> {
        <C as Selection<CTX>>::build(&self.col, ctx)?;
        write!(ctx, " AS ")?;
        self.alias.build(ctx)?;
        Ok(())
    }
}

impl<C, A, CTX: Context> Column<CTX> for ColAlias<C, A, CTX>
where
    C: Column<CTX>,
    A: Alias<CTX>,
{
    fn build(&self, ctx: &mut CTX) -> Result<(), Error> {
        self.alias.build(ctx)?;
        Ok(())
    }
}
