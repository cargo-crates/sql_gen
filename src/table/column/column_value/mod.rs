pub mod boolean;

pub use boolean::Boolean;

use crate::error::SqlError;

#[derive(Clone, Debug)]
pub enum ColumnValue {
  Boolean(Boolean),
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

impl ColumnValue {
  pub fn to_sql_string(&self) -> Result<String, SqlError> {
    Ok("".to_owned())
  }
}