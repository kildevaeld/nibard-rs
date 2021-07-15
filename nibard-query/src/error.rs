use std::fmt::Error as FormatError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("format")]
    Format(#[from] FormatError),
}
