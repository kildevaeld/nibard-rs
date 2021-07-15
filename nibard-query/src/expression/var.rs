use super::{Expression, IntoExpression};
use crate::{Context, Error};
use nibard_shared::{Value, ValueRef};

pub struct VarExpr {
    value: Value,
}

impl Expression for VarExpr {
    fn build(&self, ctx: &mut Context) -> Result<(), Error> {
        ctx.push(self.value.clone())?;
        Ok(())
    }
}

impl IntoExpression for i32 {
    type Expression = VarExpr;
    fn into_expression(self) -> Self::Expression {
        VarExpr {
            value: Value::Int(self),
        }
    }
}

impl<'a> IntoExpression for &'a str {
    type Expression = VarExpr;
    fn into_expression(self) -> Self::Expression {
        VarExpr {
            value: Value::Text(self.to_string()),
        }
    }
}

impl IntoExpression for Value {
    type Expression = VarExpr;
    fn into_expression(self) -> Self::Expression {
        VarExpr { value: self }
    }
}

impl<'a> IntoExpression for ValueRef<'a> {
    type Expression = VarExpr;
    fn into_expression(self) -> Self::Expression {
        VarExpr { value: self.into() }
    }
}
