use super::{Context, Error, Expression, Joinable, SelectFilter, Selection, Statement, Target};
use std::fmt::Write;
pub trait Select {
    type Target: Target;
    type Selection: Selection;
    fn build(&self, ctx: &mut Context) -> Result<(), Error>;
}

pub struct Sel<T: Target, S: Selection>(pub T, pub S);

impl<T: Target, S: Selection> Select for Sel<T, S> {
    type Target = T;
    type Selection = S;
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        ctx.write_str("SELECT ")?;
        self.1.build(ctx)?;
        ctx.write_str(" FROM ")?;
        self.0.build(ctx)?;
        Ok(())
    }
}

pub trait SelectExt: Select {
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
