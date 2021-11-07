use crate::{query::Selection, Context, Error, Statement};
use nibard_shared::{Value, ValueRef};
use std::borrow::Cow;
use std::fmt::Write;

#[derive(Debug)]
pub struct Insert<'a> {
    pub(crate) table: Cow<'a, str>,
    pub(crate) keys: Vec<Cow<'a, str>>,
    pub(crate) values: Vec<Value>,
}

impl<'a> Insert<'a> {
    pub fn new(table: impl Into<Cow<'a, str>>) -> Insert<'a> {
        Insert {
            table: table.into(),
            values: Vec::default(),
            keys: Vec::default(),
        }
    }
    // pub fn set<'b: 'a, V: Into<ValueRef<'a>>>(
    //     mut self,
    //     field: impl Into<Cow<'a, str>>,
    //     value: V,
    // ) -> Self {
    //     self.keys.push(field.into());
    //     self.values.push(value.into());
    //     self
    // }

    pub fn set<V: Into<Value>>(mut self, field: impl Into<Cow<'a, str>>, value: V) -> Self {
        self.keys.push(field.into());
        self.values.push(value.into());
        self
    }

    pub fn returning<C: Context, S>(self, selection: S) -> InsertReturning<'a, S>
    where
        S: Selection<C>,
    {
        InsertReturning {
            insert: self,
            returning: selection,
        }
    }
}

impl<'a, C: Context> Statement<C> for Insert<'a> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        write!(
            ctx,
            "INSERT INTO {} ({}) VALUES (",
            self.table,
            self.keys.join(", ")
        )?;
        for (idx, value) in self.values.iter().enumerate() {
            if idx > 0 {
                ctx.write_str(",")?;
            }
            ctx.push(value.clone().into())?;
        }
        ctx.write_str(")")?;
        Ok(())
    }
}

pub struct InsertReturning<'a, S> {
    insert: Insert<'a>,
    returning: S,
}

impl<'a, S, C: Context> Statement<C> for InsertReturning<'a, S>
where
    S: Selection<C>,
{
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        self.insert.build(ctx)?;
        write!(ctx, " RETURNING ")?;
        self.returning.build(ctx)?;
        Ok(())
    }
}

pub fn insert<'a>(table: impl Into<Cow<'a, str>>) -> Insert<'a> {
    Insert::new(table)
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::build::*;
    use nibard_shared::Dialect;

    #[test]
    fn test() {
        let mut output = crate::build(Dialect::Sqlite, Insert::new("blogs").set("name", "Rasmus"));

        println!("oUTPUT {:?}", output);
    }
}
