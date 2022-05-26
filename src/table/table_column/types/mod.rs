use super::{ColumnTypeBoolean, ColumnTypeInteger, ColumnTypeDate, ColumnTypeTime, ColumnTypeDateTime};

#[derive(Clone, Debug)]
pub enum Types {
  Boolean(ColumnTypeBoolean),
  Integer(ColumnTypeInteger),
  Float,
  Double,
  Binary,
  String,
  Text,
  Date(ColumnTypeDate),
  Time(ColumnTypeTime),
  Datetime(ColumnTypeDateTime),
  Null,
}