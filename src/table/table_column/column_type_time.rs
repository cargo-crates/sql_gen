
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct ColumnTypeTime {
  precision: usize,
  default: SystemTime,
  null: bool,
  comment: &'static str,
}