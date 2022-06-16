// mysql: int(bypes)  int(1) => tinyint, int(2) => smallint, int(4) => int, int(8) => bigint
use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct Integer {
  limit: Option<u8>,
  // mysql: m => 1..=64, postgresql: width => 1...
  bit_type: Option<u8>,
  unsigned: Option<bool>,
  // must give unsigned
  zerofill: Option<bool>,

  default: Option<i64>,
  null: Option<bool>,
  comment: Option<&'static str>,
  index: Option<bool>,
  unique: Option<bool>,
  auto_increment: Option<bool>,
  primary_key: Option<bool>,
  foreign_key: Option<crate::define_table::ForeignKey>,
}

impl Default for Integer {
  fn default() -> Integer {
    Integer {
      limit: None,
      bit_type: None,
      unsigned: None,
      zerofill: None,
      default: None,
      null: None,
      comment: None,
      index: None,
      unique: None,
      auto_increment: None,
      primary_key: None,
      foreign_key: None,
    }
  }
}

impl Integer {
  pub fn set_limit(&mut self, limit: u8) -> &mut Self {
    self.limit = Some(limit);
    self
  }
  pub fn set_tinyint_type(&mut self) -> &mut Self {
    self.limit = Some(1); self
  }
  pub fn set_bigint_type(&mut self) -> &mut Self {
    self.limit = Some(8); self
  }
  pub fn set_bit_type(&mut self, bit_width: u8) -> &mut Self {
    self.bit_type = Some(bit_width);
    self
  }
  pub fn set_unsigned(&mut self, unsigned: bool) -> &mut Self {
    self.unsigned = Some(unsigned);
    self
  }
  #[cfg(feature = "mysql")]
  pub fn set_zerofill(&mut self, zerofill: bool) -> &mut Self {
    self.zerofill = Some(zerofill);
    self
  }
  pub fn set_default(&mut self, default: i64) -> &mut Self {
    self.default = Some(default);
    self
  }
  pub fn set_auto_increment(&mut self, auto_increment: bool) -> &mut Self {
    self.auto_increment = Some(auto_increment);
    self
  }
}

impl ColumnTypeable for Integer {
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

  fn foreign_key(&self) -> Option<&crate::define_table::ForeignKey> { self.foreign_key.as_ref() }
  fn set_foreign_key(&mut self, foreign_key: crate::define_table::ForeignKey) -> &mut Self {
    self.foreign_key = Some(foreign_key);
    self
  }

  fn to_sql(&self, _column: &crate::Column, _table: &crate::DefineTable) -> Option<Sql> {
    let mut sql = Sql::default();
    let mut r#type = crate::const_data::GLOBAL_DB_KEY_MAPPING.get("integer").unwrap().to_owned();
    if let Some(limit) = self.limit {
      if limit == 1 {
        r#type = "TINYINT".to_owned();
      }
      else if limit == 2 { r#type = "SMALLINT".to_owned(); }
      else if limit == 4 { r#type = "INT".to_owned(); }
      else if limit == 8 { r#type = "BIGINT".to_owned(); }
    }
    if let Some(bit_width) = self.bit_type {
      r#type = format!("BIT({})", bit_width);
    }
    sql.push_value(&r#type);
    if let Some(unsigned) = self.unsigned {
      if unsigned {
        sql.push_value(" UNSIGNED");
        #[cfg(feature = "mysql")]
        if let Some(zerofill) = self.zerofill {
          if zerofill {
            sql.push_value(" ZEROFILL");
          }
        }
      }
    }
    if let Some(null) = self.null {
      sql.push_value(&format!(" {}", if null { "NULL" } else { "NOT NULL" }));
    }
    if let Some(default) = self.default {
      sql.push_value(&format!(" DEFAULT {}", default));
    }
    if let Some(auto_increment) = self.auto_increment {
      if auto_increment {
        sql.push_value(&format!(" {}", crate::GLOBAL_DB_KEY_MAPPING.get("auto_increment").unwrap()));
      }
    }
    // if let Some(primary_key) = self.primary_key {
    //   if primary_key {
    //     sql.push_value(&format!(" PRIMARY KEY"));
    //   }
    // }
    if let Some(comment) = self.comment {
      sql.push_value(&format!(" COMMENT '{}'", comment));
    }
    Some(sql)
  }
}

impl From<Integer> for ColumnType {
  fn from(integer: Integer) -> Self {
    ColumnType::Integer(integer)
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
          table.add_column_integer("id", |integer| {
            integer.set_null(false).set_primary_key(true).set_auto_increment(true);
          });
          table.add_column_integer("integer_a", |_integer| {});
          table.add_column_integer("integer_b", |integer| {
            integer.set_default(1).set_comment("my comment");
          });
        }).try_into().unwrap();
        assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (
id INT NOT NULL AUTO_INCREMENT,
integer_a INT,
integer_b INT DEFAULT 1 COMMENT 'my comment',
PRIMARY KEY pk_on_id (id)
);".to_owned());

        // update table
        let sql: String = crate::SqlGen::alter_table("users", |table| {
          table.add_column_integer("integer_a", |_integer| {});
          table.add_column_integer("integer_b", |integer| {
            integer.set_default(1).set_comment("my comment");
          });
          table.modify_column_integer("integer_c", |integer| {
            integer.set_null(false).set_index(true).set_unsigned(true).set_limit(8);
          });
          table.change_column_integer("is_a", "is_b", |integer| {
            integer.set_comment("rename to is_b");
          });
          table.drop_column_integer("integer_d");
        }).try_into().unwrap();
        assert_eq!(sql, "ALTER TABLE users
ADD COLUMN integer_a INT,
ADD COLUMN integer_b INT DEFAULT 1 COMMENT 'my comment',
MODIFY COLUMN integer_c BIGINT UNSIGNED NOT NULL,
CHANGE COLUMN is_a is_b INT COMMENT 'rename to is_b',
DROP COLUMN integer_d;".to_owned());

      }
    }
}