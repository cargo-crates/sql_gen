use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SqlError {
  #[error("generate sql error info")]
  Message(String),
  #[error("unknown data store error")]
  Unknown,
}