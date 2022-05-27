
// https://docs.rs/sqlx/0.5.1/sqlx/mysql/types/index.html

use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct Timestamp {
  // default_utc: Option<chrono::DateTime<chrono::Utc>>,
  // default_local: Option<chrono::DateTime<chrono::Local>>,
  default: Option<String>,
  null: Option<bool>,
  comment: Option<&'static str>,
  index: Option<bool>,
  unique: Option<bool>,
  primary_key: Option<bool>,
  on_create_current_timestamp: Option<bool>,
  on_update_current_timestamp: Option<bool>,
}

impl Default for Timestamp {
  fn default() -> Timestamp {
    Timestamp {
      default: None,
      null: None,
      comment: None,
      index: None,
      unique: None,
      primary_key: None,
      on_create_current_timestamp: None,
      on_update_current_timestamp: None,
    }
  }
}

impl Timestamp {
  pub fn set_default(&mut self, default: &str) -> &mut Self {
    self.default = Some(default.to_owned());
    self
  }
  // pub fn set_default_utc(&mut self, default: chrono::DateTime<chrono::Utc>) -> &mut Self {
  //   self.default_utc = Some(default);
  //   self
  // }
  pub fn set_on_create_current_timestamp(&mut self, on_create_current_timestamp: bool) -> &mut Self {
    self.on_create_current_timestamp = Some(on_create_current_timestamp);
    self
  }
  pub fn set_on_update_current_timestamp(&mut self, on_update_current_timestamp: bool) -> &mut Self {
    self.on_update_current_timestamp = Some(on_update_current_timestamp);
    self
  }
}

impl ColumnTypeable for Timestamp {
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
    let r#type = crate::const_data::GLOBAL_DB_KEY_MAPPING.get("timestamp").unwrap();
    sql.push_value(r#type);
    if let Some(null) = self.null {
      sql.push_value(&format!(" {}", if null { "NULL" } else { "NOT NULL" }));
    }
    // if let Some(ref default) = self.default_utc {
    //   sql.push_value(&format!(" DEFAULT {}", default));
    // } else if let Some(default) = self.default_local {
    //   sql.push_value(&format!(" DEFAULT {}", default));
    // }
    if let Some(on_create_current_timestamp) = self.on_create_current_timestamp {
      if on_create_current_timestamp {
        sql.push_value(&format!(" DEFAULT {}", "CURRENT_TIMESTAMP"));  
      }
    } else if let Some(ref default) = self.default {
      sql.push_value(&format!(" DEFAULT {}", default));
    }

    if let Some(primary_key) = self.primary_key {
      if primary_key {
        sql.push_value(&format!(" PRIMARY KEY"));
      }
    }
    if let Some(on_update_current_timestamp) = self.on_update_current_timestamp {
      if on_update_current_timestamp {
        sql.push_value(&format!(" ON UPDATE CURRENT_TIMESTAMP"));
      }
    }
    if let Some(comment) = self.comment {
      sql.push_value(&format!(" COMMENT '{}'", comment));
    }
    Some(sql)
  }
}

impl From<Timestamp> for ColumnType {
  fn from(date: Timestamp) -> Self {
    ColumnType::Timestamp(date)
  }
}


#[cfg(test)]

mod tests {
  use crate::prelude::*;
  // use chrono::TimeZone;
  #[test]
  fn to_sql() {
    #[cfg(feature = "mysql")]
    {
      // create table
      let sql: String = crate::SqlGen::create_table("users", |table| {
        table.add_column_timestamp("timestamp_at_a", |_timestamp| {});
        table.add_column_timestamp("timestamp_at_b", |timestamp| {
          timestamp.set_default("2022-01-01 00:00:00 +08:00").set_comment("my comment");
        });
        table.add_column_timestamp("timestamp_at_c", |timestamp| {
          timestamp.set_default("2022-01-01 00:00:00 UTC");
        });
        table.add_column_timestamp("timestamp_at_d", |timestamp| {
          timestamp.set_on_create_current_timestamp(true).set_on_update_current_timestamp(true);
        });
      }).try_into().unwrap();
      assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (
timestamp_at_a TIMESTAMP,
timestamp_at_b TIMESTAMP DEFAULT 2022-01-01 00:00:00 +08:00 COMMENT 'my comment',
timestamp_at_c TIMESTAMP DEFAULT 2022-01-01 00:00:00 UTC,
timestamp_at_d TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);".to_owned());

      // update table
      let sql: String = crate::SqlGen::alter_table("users", |table| {
        table.add_column_timestamp("timestamp_at_a", |_timestamp| {});
        table.add_column_timestamp("timestamp_at_b", |timestamp| {
          timestamp.set_default("2022-01-01 00:00:00 +08:00").set_comment("my comment");
        });
        table.modify_column_timestamp("", |timestamp| {
          timestamp.set_null(false).set_index(true);
        });
        table.change_column_timestamp("is_a", "is_b", |timestamp| {
          timestamp.set_comment("rename to is_b");
        });
        table.drop_column_timestamp("timestamp_at_c");
      }).try_into().unwrap();
      assert_eq!(sql, "ALTER TABLE users
ADD COLUMN timestamp_at_a TIMESTAMP,
ADD COLUMN timestamp_at_b TIMESTAMP DEFAULT 2022-01-01 00:00:00 +08:00 COMMENT 'my comment',
MODIFY COLUMN  TIMESTAMP NOT NULL,
CHANGE COLUMN is_a is_b TIMESTAMP COMMENT 'rename to is_b',
DROP COLUMN timestamp_at_c;".to_owned());

    }
  }
}