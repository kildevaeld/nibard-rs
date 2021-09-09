mod executor_ext;
mod query;

pub use nibard_connection as connection;
pub use nibard_connection::*;
pub use nibard_query::*;
pub use nibard_shared::*;

pub mod prelude {
    pub use super::executor_ext::*;
    pub use super::query::StatementQuery;
    pub use nibard_connection::{Execute, Executor, Row, RowExt};
    pub use nibard_query::{
        ColExt, Column, Expression, ExpressionExt, Select, SelectExt, Table, TableExt, Target,
        TargetExt,
    };
}
