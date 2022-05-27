
// https://docs.rs/sqlx/0.5.1/sqlx/mysql/types/index.html

use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct Datetime {
  // default: Option<chrono::NaiveDateTime>,
  default: Option<String>,
  null: Option<bool>,
  comment: Option<&'static str>,
  index: Option<bool>,
  unique: Option<bool>,
  primary_key: Option<bool>,
}

impl Default for Datetime {
  fn default() -> Datetime {
    Datetime {
      default: None,
      null: None,
      comment: None,
      index: None,
      unique: None,
      primary_key: None,
    }
  }
}

impl Datetime {
  pub fn set_default(&mut self, default: &str) -> &mut Self {
    self.default = Some(default.to_owned());
    self
  }
}

impl ColumnTypeable for Datetime {
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

  fn to_sql(&self, _column: &crate::Column, _table: &crate::Table) -> Option<Sql> {
    let mut sql = Sql::default();
    let r#type = crate::const_data::GLOBAL_DB_KEY_MAPPING.get("datetime").unwrap();
    sql.push_value(r#type);
    if let Some(null) = self.null {
      sql.push_value(&format!(" {}", if null { "NULL" } else { "NOT NULL" }));
    }
    if let Some(ref default) = self.default {
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

impl From<Datetime> for ColumnType {
  fn from(date: Datetime) -> Self {
    ColumnType::Datetime(date)
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
          table.add_column_datetime("datetime_at_a", |_datetime| {});
          table.add_column_datetime("datetime_at_b", |datetime| {
            datetime.set_default("2022-01-01 00:00:00").set_comment("my comment");
          });
        }).try_into().unwrap();
        assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (
datetime_at_a DATETIME,
datetime_at_b DATETIME DEFAULT 2022-01-01 00:00:00 COMMENT 'my comment'
);".to_owned());

        // update table
        let sql: String = crate::SqlGen::alter_table("users", |table| {
          table.add_column_datetime("datetime_at_a", |_datetime| {});
          table.add_column_datetime("datetime_at_b", |datetime| {
            datetime.set_default("2022-01-01 00:00:00").set_comment("my comment");
          });
          table.modify_column_datetime("", |datetime| {
            datetime.set_null(false).set_index(true);
          });
          table.change_column_datetime("is_a", "is_b", |datetime| {
            datetime.set_comment("rename to is_b");
          });
          table.drop_column_datetime("datetime_at_c");
        }).try_into().unwrap();
        assert_eq!(sql, "ALTER TABLE users
ADD COLUMN datetime_at_a DATETIME,
ADD COLUMN datetime_at_b DATETIME DEFAULT 2022-01-01 00:00:00 COMMENT 'my comment',
MODIFY COLUMN  DATETIME NOT NULL,
CHANGE COLUMN is_a is_b DATETIME COMMENT 'rename to is_b',
DROP COLUMN datetime_at_c;".to_owned());

      }
    }
}