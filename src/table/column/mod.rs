pub mod column_type;
pub mod column_value;

pub use column_type::{ColumnType, ColumnTypeAction, Boolean, Integer};
pub use column_value::ColumnValue;

use crate::collectors::Sql;

#[derive(Clone, Debug)]
pub struct Column {
  // column_name: String,
  pub column_names: Vec<String>,
  pub column_type_action: ColumnTypeAction,
  column_type: ColumnType,
}

impl Column {
  pub fn new(column_name: &str, column_type_action: ColumnTypeAction, column_type: ColumnType) -> Column {
    Column {
      // column_name: column_name.to_owned(),
      column_names: vec![column_name.to_owned()],
      column_type_action,
      column_type
    }
  }
  pub fn column_name(&self) -> &str {
    self.column_names.get(0).unwrap()
  }
  pub fn to_sql(&self, table: &crate::Table) -> Option<Sql> {
    let mut final_ret = None;
    match &self.column_type_action {
      ColumnTypeAction::AddColumn { position } | ColumnTypeAction::ModifyColumn { position } => {
        if let Some(type_sql) = self.column_type.to_sql(self, table) {
          let mut sql = Sql::new(format!("{}", self.column_name()));
          sql.push(' ').push_sql(&type_sql);
          if let Some(position) = position {
            sql.push(' ').push_value(&position);
          }
          final_ret =Some(sql)
        }
      },
      ColumnTypeAction::ChangeColumn { new_name, position, } => {
        if let Some(type_sql) = self.column_type.to_sql(self, table) {
          let mut sql = Sql::new(format!("{}", self.column_name()));
          sql.push_value(&format!(" {}", new_name));
          sql.push(' ').push_sql(&type_sql);
          if let Some(position) = position {
            sql.push(' ').push_value(&position);
          }
          final_ret =Some(sql)
        }
      },
      ColumnTypeAction::RenameColumn { new_name, position, } => {
        let mut sql = Sql::new(format!("{}", self.column_name()));
        sql.push_value(&format!(" TO {}", new_name));
        if let Some(position) = position {
          sql.push(' ').push_value(&position);
        }
        final_ret =Some(sql)
      },
      ColumnTypeAction::AddConstraint => {
        if let Some(type_sql) = self.column_type.to_sql(self, table) {
          let mut sql = Sql::new(format!("{}", self.column_name()));
          sql.push(' ').push_sql(&type_sql);
          final_ret =Some(sql)
        }
      },
      ColumnTypeAction::DropColumn | ColumnTypeAction::DropConstraint => {
        let sql = Sql::new(format!("{}", self.column_name()));
        final_ret =Some(sql)
      },
      ColumnTypeAction::RenameIndex { new_name, } => {
        let mut sql = Sql::new(format!("{}", self.column_name()));
        sql.push_value(&format!(" TO {}", new_name));
        final_ret =Some(sql)
      },
    }
    final_ret
  }
  pub fn to_constraint_sql(&self, table: &crate::Table) -> Option<Sql> {
    self.column_type.to_constraint_sql(self, table)
  }
}