use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct ColumnString {
  // varchar max 65535, cahr: max 255, default: varchar
  is_char: Option<bool>,
  length: Option<u32>,
  default: Option<String>,
  null: Option<bool>,
  comment: Option<&'static str>,
  index: Option<bool>,
  unique: Option<bool>,
  primary_key: Option<bool>,
}

impl Default for ColumnString {
  fn default() -> ColumnString {
    ColumnString {
      is_char: None,
      length: Some(255),
      default: None,
      null: None,
      comment: None,
      index: None,
      unique: None,
      primary_key: None,
    }
  }
}

impl ColumnString {
  pub fn set_char_type(&mut self, length: u8) -> &mut Self {
    self.is_char = Some(true);
    self.length = Some(length.into());
    self
  }
  pub fn set_length(&mut self, length: u32) -> &mut Self {
    self.length = Some(length);
    self
  }
  pub fn set_default(&mut self, default: &str) -> &mut Self {
    self.default = Some(default.to_owned());
    self
  }
}

impl ColumnTypeable for ColumnString {
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

    let mut r#type = format!("{}({})", crate::const_data::GLOBAL_DB_KEY_MAPPING.get("string").unwrap(), self.length.unwrap());
    if let Some(is_char) = self.is_char {
      if is_char {
        r#type = format!("CHAR({})", self.length.unwrap());
      }
    }
    sql.push_value(&r#type);

    if let Some(null) = self.null {
      sql.push_value(&format!(" {}", if null { "NULL" } else { "NOT NULL" }));
    }
    if let Some(default) = &self.default {
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


impl From<ColumnString> for ColumnType {
  fn from(string: ColumnString) -> Self {
    ColumnType::String(string)
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
          table.add_column_string("string_a", |_string| {});
          table.add_column_string("string_b", |string| {
            string.set_default("1").set_comment("my comment");
          });
        }).try_into().unwrap();
        assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (
string_a VARCHAR(255),
string_b VARCHAR(255) DEFAULT 1 COMMENT 'my comment'
);".to_owned());

        // update table
        let sql: String = crate::SqlGen::alter_table("users", |table| {
          table.add_column_string("string_a", |_string| {});
          table.add_column_string("string_b", |string| {
            string.set_default("1").set_comment("my comment");
          });
          table.modify_column_string("string_c", |string| {
            string.set_null(false).set_index(true);
          });
          table.change_column_string("is_a", "is_b", |string| {
            string.set_comment("rename to is_b");
          });
          table.drop_column_string("string_d");
        }).try_into().unwrap();
        assert_eq!(sql, "ALTER TABLE users
ADD COLUMN string_a VARCHAR(255),
ADD COLUMN string_b VARCHAR(255) DEFAULT 1 COMMENT 'my comment',
MODIFY COLUMN string_c VARCHAR(255) NOT NULL,
CHANGE COLUMN is_a is_b VARCHAR(255) COMMENT 'rename to is_b',
DROP COLUMN string_d;".to_owned());

      }
    }
}