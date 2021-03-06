mod executor_ext;
pub mod query;

pub use nibard_connection as connection;
pub use nibard_connection::*;
pub use nibard_shared::*;

pub mod prelude {
    pub use super::executor_ext::*;
    pub use super::query::StatementQuery;
    pub use nibard_connection::{Execute, Executor, Row, RowExt};
}

pub use nibard_dsl as dsl;

// pub mod dsl {
//     pub use nibard_dsl::*;
//     pub mod prelude {
//         pub use nibard_dsl::{
//             ColExt, Column, Expression, ExpressionExt, Select, SelectExt, Table, TableExt, Target,
//             TargetExt,
//         };
//     }
// }
