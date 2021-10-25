use crate::{query::Expression, Context, Error, Statement};
use nibard_shared::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;

// #[derive(Debug)]
pub struct Update<'a> {
    pub(crate) table: Cow<'a, str>,
    pub(crate) values: HashMap<Cow<'a, str>, Value>,
    pub(crate) filters: Option<Box<dyn Expression>>,
}

impl<'a> Update<'a> {
    pub fn new(table: impl Into<Cow<'a, str>>) -> Update<'a> {
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

    pub fn on<E: Expression + 'static>(mut self, e: E) -> Self {
        self.filters = Some(Box::new(e));
        self
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<'a> Statement for Update<'a> {
    fn build(&self, ctx: &mut Context<'_>) -> Result<(), Error> {
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

pub fn update<'a>(table: impl Into<Cow<'a, str>>) -> Update<'a> {
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
        let mut ctx = Context::new(Dialect::Sqlite, &mut output);

        let t = Update::new("blogs").set("name", "Rasmus").on("id".eql(1));

        t.build(&mut ctx).unwrap();
        println!("UPDATE {}", output);
    }
}
