mod column;
mod column_ext;
mod expression;
mod filter;
mod join;
mod select;
mod selection;
mod table;
mod target;
mod types;

pub use self::{
    column::*, column_ext::*, column_ext::*, expression::*, filter::*, join::*, select::*,
    selection::*, table::*, target::*, types::*,
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::DefaultContext;
    use nibard_shared::Dialect;

    #[test]
    fn test() {
        // let mut output = String::default();
        // {
        //     let ctx = Context::new(nibard_shared::Dialect::Sqlite, &mut output);
        // let select = "todos"
        //     .select(("id", "label", "description"))
        //     .filter("test".eql("test"));

        // }
        let table = table("todos").alias("todo");
        let other = "users"; //.alias("author");

        let select = (&table,)
            .select((
                (&table).col("id"),
                (&table).col("label"),
                (&table).col("description"),
            ))
            // .join(Join::left(&other).on((&other).col("id").eql(2)))
            .filter(
                (&table)
                    .col("id")
                    .eql(1)
                    .and_group("label".like("%stuff%").and("test".eql("rapper"))),
            )
            .limit(100);
        let out = crate::build(Dialect::Sqlite, select).unwrap();
        println!("TEST {:?}", out);
    }
}