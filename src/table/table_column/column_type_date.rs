
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct ColumnTypeDate {
  precision: usize,
  default: SystemTime,
  null: bool,
  comment: &'static str,
}