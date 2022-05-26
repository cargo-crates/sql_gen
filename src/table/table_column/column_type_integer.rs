// mysql: int(limit)  int(1) => tinyint, int(2) => smallint, int(4) => int, int(8) => bigint
#[derive(Clone, Debug)]
pub struct ColumnTypeInteger {
  limit: u8,
  unsigned: bool,
  default: i64,
  null: bool,
  comment: &'static str,
}