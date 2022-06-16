pub mod boolean;
pub mod integer;
pub mod float;
pub mod double;
pub mod decimal;
pub mod string;
pub mod text;
pub mod time;
pub mod date;
pub mod datetime;
pub mod timestamp;
pub mod json;
pub mod blob;
pub mod binary;
pub mod index;
pub mod unique;
pub mod primary_key;
pub mod foreign_key;

pub use boolean::Boolean;
pub use integer::Integer;
pub use float::Float;
pub use double::Double;
pub use decimal::Decimal;
pub use string::ColumnString;
pub use text::Text;
pub use time::Time;
pub use date::Date;
pub use datetime::Datetime;
pub use timestamp::Timestamp;
pub use json::Json;
pub use blob::Blob;
pub use binary::Binary;
pub use index::Index;
pub use unique::Unique;
pub use primary_key::PrimaryKey;
pub use foreign_key::ForeignKey;

use crate::collectors::Sql;
use crate::column::Column;

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnTypeAction {
  // mysql => postion: [ FIRST | AFTER column_name ]
  AddColumn { position: Option<String> },
  ModifyColumn { position: Option<String> },
  ChangeColumn { position: Option<String>, new_name: String },
  RenameColumn { position: Option<String>, new_name: String },
  DropColumn,
  AddConstraint,
  DropConstraint,
  RenameIndex { new_name: String },
}

#[derive(Clone, Debug)]
pub enum ColumnType {
  Boolean(Boolean),
  Integer(Integer),
  Float(Float),
  Double(Double),
  Decimal(Decimal),
  String(string::ColumnString),
  Text(Text),
  Time(Time),
  Date(Date),
  Datetime(Datetime),
  Timestamp(Timestamp),
  Json(Json),
  Blob(Blob),
  Binary(Binary),
  // Null,
  Index(Index),
  Unique(Unique),
  PrimaryKey(PrimaryKey),
  ForeignKey(ForeignKey),
  // Reference,
}

impl ColumnType {
  pub fn to_sql(&self, column: &crate::Column, table: &crate::DefineTable) -> Option<Sql> {
    match self {
      ColumnType::Boolean(boolean) => {
        return boolean.to_sql(column, table)
      },
      ColumnType::Integer(integer) => {
        return integer.to_sql(column, table)
      },
      ColumnType::Float(float) => {
        return float.to_sql(column, table)
      },
      ColumnType::Double(double) => {
        return double.to_sql(column, table)
      },
      ColumnType::Decimal(decimal) => {
        return decimal.to_sql(column, table)
      },
      ColumnType::String(string) => {
        return string.to_sql(column, table)
      },
      ColumnType::Text(text) => {
        return text.to_sql(column, table)
      },
      ColumnType::Time(time) => {
        return time.to_sql(column, table)
      },
      ColumnType::Date(date) => {
        return date.to_sql(column, table)
      },
      ColumnType::Datetime(datetime) => {
        return datetime.to_sql(column, table)
      },
      ColumnType::Timestamp(timestamp) => {
        return timestamp.to_sql(column, table)
      },
      ColumnType::Json(json) => {
        return json.to_sql(column, table)
      },
      ColumnType::Blob(blob) => {
        return blob.to_sql(column, table)
      },
      ColumnType::Binary(binary) => {
        return binary.to_sql(column, table)
      },
      ColumnType::Index(index) => {
        return index.to_sql(column, table) // None
      },
      ColumnType::Unique(unique) => {
        return unique.to_sql(column, table) // None
      },
      ColumnType::PrimaryKey(primary_key) => {
        return primary_key.to_sql(column, table) // None
      },
      ColumnType::ForeignKey(foreign_key) => {
        return foreign_key.to_sql(column, table) // None
      },
      // _ => None,
    }
  }
  pub fn to_constraint_sql(&self, column: &Column, table: &crate::DefineTable) -> Option<Sql> {
    match self {
      ColumnType::Boolean(boolean) => {
        return boolean.to_constraint_sql(column, table)
      },
      ColumnType::Integer(integer) => {
        return integer.to_constraint_sql(column, table)
      },
      ColumnType::Float(float) => {
        return float.to_constraint_sql(column, table)
      },
      ColumnType::Double(double) => {
        return double.to_constraint_sql(column, table)
      },
      ColumnType::Decimal(decimal) => {
        return decimal.to_constraint_sql(column, table)
      },
      ColumnType::String(string) => {
        return string.to_constraint_sql(column, table)
      },
      ColumnType::Text(text) => {
        return text.to_constraint_sql(column, table)
      },
      ColumnType::Time(time) => {
        return time.to_constraint_sql(column, table)
      },
      ColumnType::Date(date) => {
        return date.to_constraint_sql(column, table)
      },
      ColumnType::Datetime(datetime) => {
        return datetime.to_constraint_sql(column, table)
      },
      ColumnType::Json(json) => {
        return json.to_constraint_sql(column, table)
      },
      ColumnType::Blob(blob) => {
        return blob.to_constraint_sql(column, table)
      },
      ColumnType::Binary(binary) => {
        return binary.to_constraint_sql(column, table)
      },
      ColumnType::Timestamp(timestamp) => {
        return timestamp.to_constraint_sql(column, table)
      },
      ColumnType::Index(index) => {
        return index.to_constraint_sql(column, table)
      },
      ColumnType::Unique(unique) => {
        return unique.to_constraint_sql(column, table)
      },
      ColumnType::PrimaryKey(primary_key) => {
        return primary_key.to_constraint_sql(column, table)
      },
      ColumnType::ForeignKey(foreign_key) => {
        return foreign_key.to_constraint_sql(column, table)
      },
      // _ => None,
    }
  }
}

pub trait ColumnTypeable {
  fn null(&self) -> Option<bool> { None }
  fn set_null(&mut self, _null: bool) -> &mut Self { self }
  fn set_not_null(&mut self, not_null: bool) -> &mut Self {
    self.set_null(!not_null)
  }

  fn set_comment(&mut self, _column_name: &'static str) -> &mut Self { self }

  fn index(&self) -> Option<bool> { None }
  fn set_index(&mut self, _index: bool) -> &mut Self { self }

  fn unique(&self) -> Option<bool> { None }
  fn set_unique(&mut self, _unique: bool) -> &mut Self { self }

  fn primary_key(&self) -> Option<bool> { None }
  fn set_primary_key(&mut self, _primary_key: bool) -> &mut Self { self }

  fn foreign_key(&self) -> Option<&crate::define_table::ForeignKey> { None }
  fn set_foreign_key(&mut self, _foreign_key: crate::define_table::ForeignKey) -> &mut Self { self }

  fn to_sql(&self, column: &Column, table: &crate::DefineTable) -> Option<Sql>;
  fn to_constraint_sql(&self, column: &Column, _table: &crate::DefineTable) -> Option<Sql> {
    let mut sql = Sql::default();
    if let Some(primary_key) = self.primary_key() {
      if primary_key {
        sql.push_value(&format!("PRIMARY KEY pk_on_{}", column.column_name()));
        match column.column_type_action {
          ColumnTypeAction::AddColumn {..} | ColumnTypeAction::AddConstraint => {
            sql.push_value(&format!(" ({})", column.column_name()));
          },
          ColumnTypeAction::DropConstraint => {
            sql.push_value(&format!("DROP INDEX {}", column.column_name()));
          },
          _ => ()
        }
      }
    }
    let mut index = false;
    let mut unique = false;
    if let Some(_unique) = self.unique() { unique = _unique; }
    if let Some(_index) = self.index() { index = _index; }
    if unique || index {
      if unique && index {
        match column.column_type_action {
          ColumnTypeAction::AddColumn {..} | ColumnTypeAction::AddConstraint => {
            sql.push_value(&format!("UNIQUE INDEX unique_index_on_{}", column.column_names.join("_and_")));
            sql.push_value(&format!(" ({})", column.column_names.join(",")));
          },
          ColumnTypeAction::RenameIndex { ref new_name } => {
            sql.push_value(&format!("RENAME INDEX {} TO {}", column.column_name(), new_name));
          },
          ColumnTypeAction::DropConstraint => {
            sql.push_value(&format!("DROP INDEX {}", column.column_name()));
          },
          _ => ()
        }
      } else if unique && !index {
        match column.column_type_action {
          ColumnTypeAction::AddColumn {..} | ColumnTypeAction::AddConstraint => {
            sql.push_value(&format!("UNIQUE unique_on_{}", column.column_names.join("_and_")));
            sql.push_value(&format!(" ({})", column.column_names.join(",")));
          },
          ColumnTypeAction::RenameIndex { ref new_name } => {
            sql.push_value(&format!("RENAME INDEX {} TO {}", column.column_name(), new_name));
          },
           ColumnTypeAction::DropConstraint => {
              sql.push_value(&format!("DROP INDEX {}", column.column_name()));
            },
          _ => ()
        }
      } else if !unique && index {
        match column.column_type_action {
          ColumnTypeAction::AddColumn {..} | ColumnTypeAction::AddConstraint => {
            sql.push_value(&format!("INDEX index_on_{} ({})", column.column_names.join("_and_"), column.column_names.join(",")));
          },
          ColumnTypeAction::RenameIndex { ref new_name } => {
            sql.push_value(&format!("RENAME INDEX {} TO {}", column.column_name(), new_name));
          },
          ColumnTypeAction::DropConstraint => {
            sql.push_value(&format!("DROP INDEX {}", column.column_name()));
          },
          _ => ()
        }
      }
    }

    if let Some(foreign_key) = self.foreign_key() {
      if !sql.is_empty() { sql.push_value(",\n"); }
      sql.push_sql(&foreign_key.to_sql(&column));
    }
    if sql.is_empty() { None } else { Some(sql) }
  }
}