use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Dialect {
    Sqlite,
    Pg,
    MySQL,
}

impl fmt::Display for Dialect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = match self {
            Dialect::Sqlite => "sqlite3",
            Dialect::Pg => "postgres",
            Dialect::MySQL => "mysql",
        };
        f.write_str(m)
    }
}
