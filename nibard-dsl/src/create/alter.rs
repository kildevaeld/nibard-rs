use crate::{Context, Error, Statement};

use super::{Column, ForeignKey};
use std::borrow::Cow;
use std::fmt::Write;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AlterTable<'a> {
    pub table: Cow<'a, str>,
    pub ty: AlterTableType<'a>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlterTableType<'a> {
    Rename(Cow<'a, str>),
    AddColumn(Column<'a>),
    RemoveColumn(Cow<'a, str>),
    RenameColumn,
    ForeignKey(AlterForeignKey<'a>),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AlterForeignKey<'a> {
    pub name: Cow<'a, str>,
    pub column: Cow<'a, str>,
    pub fk: ForeignKey<'a>,
}

impl<'a, C: Context> Statement<C> for AlterTable<'a> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        write!(ctx, "ALTER TABLE {}", self.table)?;

        match &self.ty {
            AlterTableType::ForeignKey(a) => {
                //
                write!(
                    ctx,
                    " ADD CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {} ({})",
                    a.name, a.column, a.fk.table, a.fk.column
                )?;
            }
            _ => {
                unimplemented!("not implemeted")
            }
        }

        Ok(())
    }
}
