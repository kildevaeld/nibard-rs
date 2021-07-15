#[cfg(all(
    not(feature = "sqlite"),
    not(feature = "mysql"),
    not(feature = "postgres")
))]
compile_error!("need at least one");

#[macro_use]
mod macros;

mod database;
mod error;
mod executor;
mod query;
mod row;
mod transaction;

// #[cfg(feature = "serde")]
// mod de;

pub use self::{database::*, executor::*, query::*, query::*, row::*, transaction::*};

#[cfg(test)]
mod test {
    use super::*;
    use nibard_query::*;
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test() {
        let db = Database::open("sqlite::memory:").await.expect("sqlite");

        db.execute(r#"CREATE TABLE todos(id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT, description TEXT DEFAULT NULL)"#, &[]).await.expect("create table");

        db.execute(r#"INSERT INTO todos (label) VALUES ("Hello, World")"#, &[])
            .await
            .expect("insert");

        let table = &"todos"; //.alias("todo");

        let table_id = table.col("id");

        let select = (&table)
            .select((
                (&table_id).alias("todo__id"),
                (&table).col("label"),
                (&table).col("description"),
            ))
            .filter((&table_id).eql(1));

        let query = select
            .to_query(db.dialect())
            .fetch_one(&db)
            .await
            .expect("fetch one");

        println!("QUERY {:?}", query.to_map());
    }
}
