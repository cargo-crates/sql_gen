use crate::error::SqlError;
use super::{ColumnValueBoolean};

#[derive(Clone, Debug)]
pub enum Values {
  Boolean(ColumnValueBoolean),
  Integer,
  Float,
  Double,
  Binary,
  String,
  Text,
  Date,
  Time,
  Datetime,
  /// A unique identifier type
  UUID(String), // TODO: Change to UUID type
  Null,
}

impl Values {
  pub fn to_sql_string(&self) -> Result<String, SqlError> {
    Ok("".to_owned())
  }
}