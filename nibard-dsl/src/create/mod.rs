use crate::{Context, Error, Statement};
use nibard_shared::{Dialect, Type, Value};
use std::borrow::Cow;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTable<'a> {
    pub name: Cow<'a, str>,
    pub fields: Vec<Column<'a>>,
    pub force: bool,
    #[allow(unused)]
    pub temporary: bool,
}

impl<'a> Statement for CreateTable<'a> {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        ctx.write_str("CREATE TABLE ")?;
        if !self.force {
            ctx.write_str("IF NOT EXISTS ")?;
        }
        ctx.write_str(&self.name)?;
        ctx.write_str("(")?;
        let mut fks = Vec::default();
        for (i, v) in self.fields.iter().enumerate() {
            if i > 0 {
                ctx.write_str(", ")?;
            }
            v.build(ctx)?;

            if let Some(fk) = &v.foreign_key {
                fks.push((&v.name, fk));
            }
        }
        for (name, fk) in fks.into_iter() {
            write!(
                ctx,
                ", FOREIGN KEY ({}) REFERENCES {}({})",
                name, fk.table, fk.column
            )?;
        }
        ctx.write_str(")")?;
        Ok(())
    }
}

impl<'a> CreateTable<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            fields: Vec::default(),
            force: false,
            temporary: false,
        }
    }
    pub fn column(mut self, field: Column<'a>) -> Self {
        self.fields.push(field);
        self
    }

    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Column<'a> {
    pub name: Cow<'a, str>,
    pub ty: Type,
    pub required: bool,
    pub primary_key: bool,
    pub default: Option<Value>,
    pub foreign_key: Option<ForeignKey<'a>>,
}

impl<'a> Column<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, ty: Type) -> Column<'a> {
        Column {
            name: name.into(),
            ty,
            required: false,
            primary_key: false,
            default: None,
            foreign_key: None,
        }
    }

    pub fn not_null(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self
    }

    pub fn foreign_key(mut self, fk: ForeignKey<'a>) -> Self {
        self.foreign_key = Some(fk);
        self
    }
}

impl<'a> Column<'a> {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
        write!(ctx, "{} ", self.name)?;

        let dialect = ctx.dialect();

        if !(self.ty.is_auto() && ctx.dialect() == Dialect::Pg) {
            self.ty.write_sql(ctx, dialect)?;
        } else {
            ctx.write_str("SERIAL")?;
        }

        if self.primary_key {
            write!(ctx, " PRIMARY KEY")?;
        }

        if !(self.ty.is_auto() && ctx.dialect() == Dialect::Sqlite) {
            if self.required {
                ctx.write_str(" NOT NULL")?;
            } else {
                ctx.write_str(" DEFAULT NULL")?;
            }
        }

        if self.ty.is_auto() {
            if ctx.dialect() == Dialect::Sqlite {
                ctx.write_str(" AUTOINCREMENT")?;
            }
        }

        if let Some(_default) = &self.default {}

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForeignKey<'a> {
    // #[cfg_attr(feature = "serde", serde(borrow))]
    pub table: Cow<'a, str>,
    // #[cfg_attr(feature = "serde", serde(borrow))]
    pub column: Cow<'a, str>,
    on_update: ReferentialAction,
    on_delete: ReferentialAction,
}

impl<'a> ForeignKey<'a> {
    pub fn new(table: impl Into<Cow<'a, str>>, column: impl Into<Cow<'a, str>>) -> ForeignKey<'a> {
        ForeignKey {
            table: table.into(),
            column: column.into(),
            on_delete: ReferentialAction::NoAction,
            on_update: ReferentialAction::NoAction,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReferentialAction {
    Cascade,
    Restrict,
    SetNull,
    SetDefault,
    NoAction,
}
