pub mod types;
mod column_type_boolean;
mod column_type_integer;
mod column_type_date;
mod column_type_time;
mod column_type_date_time;

pub mod values;
mod column_value_boolean;

pub use types::Types;
pub use column_type_boolean::ColumnTypeBoolean;
pub use column_type_integer::ColumnTypeInteger;
pub use column_type_date::ColumnTypeDate;
pub use column_type_time::ColumnTypeTime;
pub use column_type_date_time::ColumnTypeDateTime;

pub use values::Values;
pub use column_value_boolean::ColumnValueBoolean;

use crate::collectors::Sql;

#[derive(Clone, Debug)]
pub struct TableColumn {
  column_name: String,
  column_type: Types,
}

impl TableColumn {
  pub fn to_sql(&self) -> Sql {
    Sql::default()
  }
}