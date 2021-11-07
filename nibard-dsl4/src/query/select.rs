use super::{Expression, Joinable, SelectFilter, Selection, Target};
use crate::{Context, Error, Statement};
use std::marker::PhantomData;

pub trait Select<C: Context> {
    type Target: Target<C>;
    type Selection: Selection<C>;
    fn build(&self, ctx: &mut C) -> Result<(), Error>;
}

pub type SelectBox<'a, C> = Box<
    dyn Select<C, Target = Box<dyn Target<C> + 'a>, Selection = Box<dyn Selection<C> + 'a>> + 'a,
>;

struct BoxedSelect<'a, S>(S, PhantomData<&'a dyn Fn()>);

impl<'a, S, C: Context> Select<C> for BoxedSelect<'a, S>
where
    S: Select<C> + 'a,
{
    type Target = Box<dyn Target<C> + 'a>;
    type Selection = Box<dyn Selection<C> + 'a>;
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.0.build(ctx)
    }
}

impl<'a, C: Context> Select<C> for SelectBox<'a, C> {
    type Target = Box<dyn Target<C> + 'a>;
    type Selection = Box<dyn Selection<C> + 'a>;
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

impl<'a, C: Context> Statement<C> for SelectBox<'a, C> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        (&**self).build(ctx)
    }
}

pub struct Sel<T, S> {
    pub target: T,
    pub selection: S,
}

impl<T: Target<C>, S: Selection<C>, C: Context> Select<C> for Sel<T, S> {
    type Target = T;
    type Selection = S;
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str("SELECT ")?;
        self.selection.build(ctx)?;
        ctx.write_str(" FROM ")?;
        self.target.build(ctx)?;
        Ok(())
    }
}

pub trait SelectExt<C: Context>: Select<C> {
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

    fn filter<E: Expression<C>>(self, expr: E) -> SelectFilter<Self, E>
    where
        Self: Sized,
    {
        SelectFilter(self, expr)
    }

    fn join<J: Joinable<C>>(self, join: J) -> JoinSelect<Self, J>
    where
        Self: Sized,
    {
        JoinSelect(self, join)
    }

    fn boxed<'a>(self) -> SelectBox<'a, C>
    where
        Self: Sized + 'a,
    {
        Box::new(BoxedSelect(self, PhantomData))
    }
}

impl<S, C: Context> SelectExt<C> for S where S: Select<C> {}

pub struct JoinSelect<S, J>(S, J);

impl<S, J> JoinSelect<S, J> {
    pub fn new(sel: S, join: J) -> JoinSelect<S, J> {
        JoinSelect(sel, join)
    }
}

impl<S, J, C: Context> Select<C> for JoinSelect<S, J>
where
    S: Select<C>,
    J: Joinable<C>,
{
    type Target = S::Target;
    type Selection = S::Selection;
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.0.build(ctx)?;
        ctx.write_str(" ")?;
        self.1.build(ctx)?;
        Ok(())
    }
}

impl<T: Target<C>, S: Selection<C>, C: Context> Statement<C> for Sel<T, S> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <Sel<T, S> as Select<C>>::build(self, ctx)
    }
}

impl<S: Select<C>, J: Joinable<C>, C: Context> Statement<C> for JoinSelect<S, J> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        <JoinSelect<S, J> as Select<C>>::build(self, ctx)
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

impl<S: Select<C>, C: Context> Statement<C> for LimitedSelect<S> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
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
