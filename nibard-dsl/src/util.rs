use std::fmt::{self, Write};

use nibard_shared::Dialect;

pub fn write_identifier<W>(ident: &str, dialect: &Dialect, out: &mut W) -> fmt::Result
where
    W: Write,
{
    match dialect {
        Dialect::MySQL => {
            write!(out, "`{}`", ident)
        }
        Dialect::Sqlite | Dialect::Pg => {
            write!(out, "\"{}\"", ident)
        }
    }
}

pub fn escape_identifier(ident: &str, dialect: &Dialect) -> String {
    let mut output = String::new();
    write_identifier(ident, dialect, &mut output).unwrap();
    output
}
