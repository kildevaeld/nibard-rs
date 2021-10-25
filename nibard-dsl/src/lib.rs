mod context;
mod error;
mod statement;

pub mod create;
pub mod delete;
pub mod insert;
pub mod query;
pub mod update;

pub use self::{context::*, error::Error, insert::insert, statement::Statement, update::update};

pub mod prelude {
    pub use super::query::{
        ColExt, Column, Expression, ExpressionExt, Select, SelectExt, Table, TableExt, Target,
        TargetExt,
    };
}
