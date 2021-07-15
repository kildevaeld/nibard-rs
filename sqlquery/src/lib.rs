mod column;
mod column_ext;
mod context;
mod error;
mod expression;
mod filter;
mod join;
mod select;
mod selection;
mod statement;
mod table;
mod target;
mod types;

pub use self::{
    column::*, column_ext::*, context::*, error::*, expression::*, filter::*, join::*, select::*,
    selection::*, statement::*, table::*, target::*, types::*,
};

#[cfg(test)]
mod test {
    use super::*;
    use sqlquery_shared::Dialect;

    #[test]
    fn test() {
        // let mut output = String::default();
        // {
        //     let ctx = Context::new(sqlquery_shared::Dialect::Sqlite, &mut output);
        // let select = "todos"
        //     .select(("id", "label", "description"))
        //     .filter("test".eql("test"));

        // }
        let table = "todos"; //.alias("todo");
        let other = "users"; //.alias("author");

        let select = (&table)
            .select((
                (&table).col("id"),
                (&table).col("label"),
                (&table).col("description"),
            ))
            .join(Join::left(&other).on((&other).col("id").eql(2)))
            .filter((&table).col("id").eql(1).and_group("label".like("%stuff%")));
        let out = build(Dialect::Sqlite, select).unwrap();
        println!("TEST {:?}", out);
    }
}
