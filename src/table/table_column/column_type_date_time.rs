
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct ColumnTypeDateTime {
  precision: usize,
  default: SystemTime,
  null: bool,
  comment: &'static str,
}