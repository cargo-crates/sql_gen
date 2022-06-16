use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct Double {
  default: Option<f64>,
  null: Option<bool>,
  comment: Option<&'static str>,
  index: Option<bool>,
  unique: Option<bool>,
  primary_key: Option<bool>,
}

impl Default for Double {
  fn default() -> Double {
    Double {
      default: None,
      null: None,
      comment: None,
      index: None,
      unique: None,
      primary_key: None,
    }
  }
}

impl Double {
  pub fn set_default(&mut self, default: f64) -> &mut Self {
    self.default = Some(default);
    self
  }
}

impl ColumnTypeable for Double {
  fn null(&self) -> Option<bool> { self.null }
  fn set_null(&mut self, null: bool) -> &mut Self {
    self.null = Some(null);
    self
  }

  fn set_comment(&mut self, comment: &'static str) -> &mut Self {
    self.comment = Some(comment);
    self
  }

  fn index(&self) -> Option<bool> { self.index }
  fn set_index(&mut self, index: bool) -> &mut Self { self.index = Some(index); self }

  fn unique(&self) -> Option<bool> { self.unique }
  fn set_unique(&mut self, unique: bool) -> &mut Self { self.unique = Some(unique); self }

  fn primary_key(&self) -> Option<bool> { self.primary_key }
  fn set_primary_key(&mut self, primary_key: bool) -> &mut Self {
    self.primary_key = Some(primary_key);
    self
  }

  fn to_sql(&self, _column: &crate::Column, _table: &crate::DefineTable) -> Option<Sql> {
    let mut sql = Sql::default();
    let r#type = crate::const_data::GLOBAL_DB_KEY_MAPPING.get("double").unwrap();
    sql.push_value(r#type);
    if let Some(null) = self.null {
      sql.push_value(&format!(" {}", if null { "NULL" } else { "NOT NULL" }));
    }
    if let Some(default) = self.default {
      sql.push_value(&format!(" DEFAULT {}", default));
    }
    if let Some(primary_key) = self.primary_key {
      if primary_key {
        sql.push_value(&format!(" PRIMARY KEY"));
      }
    }
    if let Some(comment) = self.comment {
      sql.push_value(&format!(" COMMENT '{}'", comment));
    }
    Some(sql)
  }
}

impl From<Double> for ColumnType {
  fn from(double: Double) -> Self {
    ColumnType::Double(double)
  }
}

#[cfg(test)]

mod tests {
  use crate::prelude::*;
    #[test]
    fn to_sql() {
      #[cfg(feature = "mysql")]
      {
        // create table
        let sql: String = crate::SqlGen::create_table("users", |table| {
          table.add_column_double("double_a", |_double| {});
          table.add_column_double("double_b", |double| {
            double.set_default(1.0).set_comment("my comment");
          });
        }).try_into().unwrap();
        assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (
double_a DOUBLE,
double_b DOUBLE DEFAULT 1 COMMENT 'my comment'
);".to_owned());

        // update table
        let sql: String = crate::SqlGen::alter_table("users", |table| {
          table.add_column_double("double_a", |_double| {});
          table.add_column_double("double_b", |double| {
            double.set_default(1.0).set_comment("my comment");
          });
          table.modify_column_double("boolean_c", |double| {
            double.set_null(false).set_index(true);
          });
          table.change_column_double("is_a", "is_b", |double| {
            double.set_comment("rename to is_b");
          });
          table.drop_column_double("double_d");
        }).try_into().unwrap();
        assert_eq!(sql, "ALTER TABLE users
ADD COLUMN double_a DOUBLE,
ADD COLUMN double_b DOUBLE DEFAULT 1 COMMENT 'my comment',
MODIFY COLUMN boolean_c DOUBLE NOT NULL,
CHANGE COLUMN is_a is_b DOUBLE COMMENT 'rename to is_b',
DROP COLUMN double_d;".to_owned());

      }
    }
}