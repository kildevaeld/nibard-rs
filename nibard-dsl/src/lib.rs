mod context;
mod error;
pub mod insert;
pub mod query;
mod statement;

pub use self::{context::*, error::Error, insert::insert, statement::Statement};

pub mod prelude {
    pub use super::query::{
        ColExt, Column, Expression, ExpressionExt, Select, SelectExt, Table, TableExt, Target,
        TargetExt,
    };
}
