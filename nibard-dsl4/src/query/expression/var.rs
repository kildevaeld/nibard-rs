use super::{Expression, IntoExpression};
use crate::{Context, Error};
use nibard_shared::{Value, ValueRef};

pub struct VarExpr {
    value: Value,
}

impl VarExpr {
    pub fn new(value: impl Into<Value>) -> VarExpr {
        VarExpr {
            value: value.into(),
        }
    }
}

impl<C: Context> Expression<C> for VarExpr {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.push(self.value.clone())?;
        Ok(())
    }
}

impl<C: Context> IntoExpression<C> for i32 {
    type Expression = VarExpr;
    fn into_expression(self) -> Self::Expression {
        VarExpr {
            value: Value::Int(self),
        }
    }
}

impl<'a, C: Context> IntoExpression<C> for &'a str {
    type Expression = VarExpr;
    fn into_expression(self) -> Self::Expression {
        VarExpr {
            value: Value::Text(self.to_string()),
        }
    }
}

impl<C: Context> IntoExpression<C> for Value {
    type Expression = VarExpr;
    fn into_expression(self) -> Self::Expression {
        VarExpr { value: self }
    }
}

impl<'a, C: Context> IntoExpression<C> for ValueRef<'a> {
    type Expression = VarExpr;
    fn into_expression(self) -> Self::Expression {
        VarExpr { value: self.into() }
    }
}
