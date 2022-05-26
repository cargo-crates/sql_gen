#[derive(Clone, Debug)]
pub struct ColumnTypeBoolean {
  default: bool,
  null: bool,
  comment: &'static str,
}