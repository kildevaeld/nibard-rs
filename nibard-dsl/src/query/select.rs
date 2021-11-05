use super::{Expression, Joinable, SelectFilter, Selection, Target};
use crate::{Context, Error, Statement};
use std::fmt::Write;
use std::marker::PhantomData;

pub trait Select {
    type Target: Target;
    type Selection: Selection;
    fn build(&self, ctx: &mut Context) -> Result<(), Error>;
}

pub type SelectBox<'a> =
    Box<dyn Select<Target = Box<dyn Target + 'a>, Selection = Box<dyn Selection + 'a>> + 'a>;

struct BoxedSelect<'a, S>(S, PhantomData<&'a dyn Fn()>);

impl<'a, S> Select for BoxedSelect<'a, S>
where
    S: Select,
{
    type Target = Box<dyn Target + 'a>;
    type Selection = Box<dyn Selection + 'a>;
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        self.0.build(ctx)
    }
}

impl<'a> Statement for SelectBox<'a> {
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

pub struct Sel<T: Target, S: Selection> {
    pub target: T,
    pub selection: S,
}

impl<T: Target, S: Selection> Sel<T, S> {}

impl<T: Target, S: Selection> Select for Sel<T, S> {
    type Target = T;
    type Selection = S;
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        ctx.write_str("SELECT ")?;
        self.selection.build(ctx)?;
        ctx.write_str(" FROM ")?;
        self.target.build(ctx)?;
        Ok(())
    }
}

pub trait SelectExt: Select {
    fn offset(self, offset: u64) -> LimitedSelect<Self>
    where
        Self: Sized,
    {
        LimitedSelect {
            sel: self,
            offset: Some(offset),
            limit: None,
        }
    }

    fn limit(self, limit: u64) -> LimitedSelect<Self>
    where
        Self: Sized,
    {
        LimitedSelect {
            sel: self,
            offset: None,
            limit: Some(limit),
        }
    }

    fn filter<E: Expression>(self, expr: E) -> SelectFilter<Self, E>
    where
        Self: Sized,
    {
        SelectFilter(self, expr)
    }

    fn join<J: Joinable>(self, join: J) -> JoinSelect<Self, J>
    where
        Self: Sized,
    {
        JoinSelect(self, join)
    }

    fn boxed<'a>(self) -> SelectBox<'a>
    where
        Self: Sized + 'a,
    {
        Box::new(BoxedSelect(self, PhantomData))
    }
}

impl<S> SelectExt for S where S: Select {}

pub struct JoinSelect<S, J>(S, J);

impl<S, J> Select for JoinSelect<S, J>
where
    S: Select,
    J: Joinable,
{
    type Target = S::Target;
    type Selection = S::Selection;
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        self.0.build(ctx)?;
        ctx.write_str(" ")?;
        self.1.build(ctx)?;
        Ok(())
    }
}

impl<T: Target, S: Selection> Statement for Sel<T, S> {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        <Sel<T, S> as Select>::build(self, ctx)
    }
}

impl<S: Select, J: Joinable> Statement for JoinSelect<S, J> {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        <JoinSelect<S, J> as Select>::build(self, ctx)
    }
}

pub struct LimitedSelect<S> {
    pub sel: S,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

impl<S> LimitedSelect<S> {
    pub fn offset(mut self, offset: impl Into<Option<u64>>) -> Self
    where
        Self: Sized,
    {
        self.offset = offset.into();
        self
    }

    pub fn limit(mut self, limit: impl Into<Option<u64>>) -> Self
    where
        Self: Sized,
    {
        self.limit = limit.into();
        self
    }
}

impl<S: Select> Statement for LimitedSelect<S> {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        self.sel.build(ctx)?;

        if let Some(limit) = self.limit {
            write!(ctx, " LIMIT {}", limit)?;
        }

        if let Some(offset) = self.offset {
            write!(ctx, " OFFSET {}", offset)?;
        }

        Ok(())
    }
}
