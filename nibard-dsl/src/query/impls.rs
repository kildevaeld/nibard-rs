use super::{Expression, IntoValue};
use crate::{Context, Error};
use nibard_shared::Value;

impl<C: Context> Expression<C> for Value {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.push(self.clone())?;
        Ok(())
    }
}

macro_rules! impl_into_value {
    ($ty: ty) => {
        impl<C: Context> IntoValue<C> for $ty {
            type Expression = Value;
            fn into_expression(self) -> Self::Expression {
                self.into()
            }
        }
    };
}

impl_into_value!(i16);
// impl_into_value!(u16);
impl_into_value!(i32);
impl_into_value!(i64);
impl_into_value!(String);
impl_into_value!(Value);

impl<'a, C: Context> IntoValue<C> for &'a str {
    type Expression = Value;
    fn into_expression(self) -> Self::Expression {
        self.into()
    }
}

impl<'a, C: Context> IntoValue<C> for &'a Value {
    type Expression = Value;
    fn into_expression(self) -> Self::Expression {
        self.clone().into()
    }
}
