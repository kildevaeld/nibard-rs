use std::borrow::Cow;
use std::fmt::Write;

use crate::{Context, Error, Statement};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateIndex<'a> {
    name: Cow<'a, str>,
    unique: bool,
    table: Cow<'a, str>,
    columns: Vec<Cow<'a, str>>,
}

impl<'a> CreateIndex<'a> {
    pub fn new(
        table: impl Into<Cow<'a, str>>,
        name: impl Into<Cow<'a, str>>,
        columns: Vec<Cow<'a, str>>,
    ) -> CreateIndex<'a> {
        CreateIndex {
            name: name.into(),
            table: table.into(),
            unique: false,
            columns,
        }
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
}

impl<'a, C: Context> Statement<C> for CreateIndex<'a> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.write_str("CREATE ")?;
        if self.unique {
            ctx.write_str("UNIQUE ")?;
        }
        write!(
            ctx,
            "INDEX IF NOT EXISTS {} ON {} ({})",
            self.name,
            self.table,
            self.columns.join(", ")
        )?;
        Ok(())
    }
}
