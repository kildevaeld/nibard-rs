use crate::{query::Expression, Context, Error, Statement};
use nibard_shared::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;

// #[derive(Debug)]
pub struct Update<'a, C: Context> {
    pub(crate) table: Cow<'a, str>,
    pub(crate) values: HashMap<Cow<'a, str>, Value>,
    pub(crate) filters: Option<Box<dyn Expression<C> + Send>>,
}

impl<'a, C: Context> Update<'a, C> {
    pub fn new(table: impl Into<Cow<'a, str>>) -> Update<'a, C> {
        Update {
            table: table.into(),
            values: HashMap::default(),
            filters: None,
        }
    }
    pub fn set<'b: 'a, V: Into<Value>>(mut self, field: impl Into<Cow<'a, str>>, value: V) -> Self {
        self.values.insert(field.into(), value.into());
        self
    }

    pub fn on<E: Expression<C> + 'static + Send>(mut self, e: E) -> Self {
        self.filters = Some(Box::new(e));
        self
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<'a, C: Context> Statement<C> for Update<'a, C> {
    fn build(&self, ctx: &mut C) -> Result<(), Error> {
        write!(ctx, "UPDATE {} SET ", self.table)?;
        for (idx, value) in self.values.iter().enumerate() {
            if idx > 0 {
                ctx.write_str(",")?;
            }
            write!(ctx, "{} = ", value.0)?;
            ctx.push(value.1.clone().into())?;
        }
        if let Some(filter) = &self.filters {
            write!(ctx, " WHERE ")?;
            filter.build(ctx)?;
        }

        Ok(())
    }
}

pub fn update<'a, C: Context>(table: impl Into<Cow<'a, str>>) -> Update<'a, C> {
    Update::new(table)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    use nibard_shared::Dialect;

    #[test]
    fn test() {
        let mut output = String::new();
        let mut ctx = crate::build(
            Dialect::Sqlite,
            Update::new("blogs").set("name", "Rasmus").on("id".eql(1)),
        );

        println!("UPDATE {:?}", ctx);
    }
}
