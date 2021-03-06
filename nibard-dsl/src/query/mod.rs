mod column_ext;
mod condition;
mod func;
mod impls;
mod join;
mod select;
mod table_ext;
mod types;

pub use self::{column_ext::*, condition::*, func::*, join::*, select::*, table_ext::*, types::*};

#[cfg(test)]
mod test {
    pub use super::*;
    pub use crate::{Context, DefaultContext, Error};
    use nibard_shared::Dialect;

    #[test]
    fn test() {
        let mut out = DefaultContext::new(Dialect::Sqlite);
        let sql = "proifles"
            .select(("id", "name".column_alias("profile_name"), Func::count_all()))
            .join(Join::left("test").on("test".col("id").eql("profile.id".expr())))
            .boxed()
            .filter(
                "name"
                    .column_alias("profile_name")
                    .eql(20)
                    .and("test".has("test".select("id").filter("test".eql(200)).expr())),
            )
            .or("test.id".eql(100))
            .offset(10)
            .limit(10)
            .build(&mut out)
            .unwrap();

        println!("Out: {:?}", out.build());
    }
}
