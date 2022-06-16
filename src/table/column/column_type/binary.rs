use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct Binary {
  // default: varbinary
  is_binary: Option<bool>,
  byte: Option<u32>,
  default: Option<Vec<u8>>,
  null: Option<bool>,
  comment: Option<&'static str>,
  index: Option<bool>,
  unique: Option<bool>,
  primary_key: Option<bool>,
}

impl Default for Binary {
  fn default() -> Binary {
    Binary {
      is_binary: None,
      byte: Some(255),
      default: None,
      null: None,
      comment: None,
      index: None,
      unique: None,
      primary_key: None,
    }
  }
}

impl Binary {
  pub fn set_binary_type(&mut self, byte: u32) -> &mut Self {
    self.is_binary = Some(true);
    self.byte = Some(byte);
    self
  }
  pub fn set_byte(&mut self, byte: u32) -> &mut Self {
    self.byte = Some(byte);
    self
  }
  pub fn set_default(&mut self, default: Vec<u8>) -> &mut Self {
    self.default = Some(default.to_owned());
    self
  }
}

impl ColumnTypeable for Binary {
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

    let mut r#type = format!("{}({})", crate::const_data::GLOBAL_DB_KEY_MAPPING.get("binary").unwrap(), self.byte.unwrap());
    if let Some(is_binary) = self.is_binary {
      if is_binary {
        r#type = format!("BINARY({})", self.byte.unwrap());
      }
    }
    sql.push_value(&r#type);

    if let Some(null) = self.null {
      sql.push_value(&format!(" {}", if null { "NULL" } else { "NOT NULL" }));
    }
    if let Some(default) = &self.default {
      sql.push_value(&format!(" DEFAULT {:?}", default));
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


impl From<Binary> for ColumnType {
  fn from(binary: Binary) -> Self {
    ColumnType::Binary(binary)
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
          table.add_column_binary("binary_a", |_binary| {});
          table.add_column_binary("binary_b", |binary| {
            binary.set_default(vec![1]).set_comment("my comment");
          });
        }).try_into().unwrap();
        assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (
binary_a VARBINARY(255),
binary_b VARBINARY(255) DEFAULT [1] COMMENT 'my comment'
);".to_owned());

        // update table
        let sql: String = crate::SqlGen::alter_table("users", |table| {
          table.add_column_binary("binary_a", |_binary| {});
          table.add_column_binary("binary_b", |binary| {
            binary.set_default(vec![1]).set_comment("my comment");
          });
          table.modify_column_binary("binary_c", |binary| {
            binary.set_null(false).set_index(true);
          });
          table.change_column_binary("is_a", "is_b", |binary| {
            binary.set_binary_type(200).set_comment("rename to is_b");
          });
          table.drop_column_binary("binary_d");
        }).try_into().unwrap();
        assert_eq!(sql, "ALTER TABLE users
ADD COLUMN binary_a VARBINARY(255),
ADD COLUMN binary_b VARBINARY(255) DEFAULT [1] COMMENT 'my comment',
MODIFY COLUMN binary_c VARBINARY(255) NOT NULL,
CHANGE COLUMN is_a is_b BINARY(200) COMMENT 'rename to is_b',
DROP COLUMN binary_d;".to_owned());

      }
    }
}