use sqlx::Error as SqlxError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("sqlx error")]
    Sqlx(#[from] SqlxError),
}
