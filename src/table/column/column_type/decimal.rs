use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct Decimal {
  precision: Option<u8>,
  scale: Option<u8>,
  default: Option<f64>,
  null: Option<bool>,
  comment: Option<&'static str>,
  index: Option<bool>,
  unique: Option<bool>,
  primary_key: Option<bool>,
}

impl Default for Decimal {
  fn default() -> Decimal {
    Decimal {
      precision: Some(30),
      scale: Some(6),
      default: None,
      null: None,
      comment: None,
      index: None,
      unique: None,
      primary_key: None,
    }
  }
}

impl Decimal {
  pub fn set_precision_scale(&mut self, precision: u8, scale: u8) -> &mut Self {
    self.precision = Some(precision);
    self.scale = Some(scale);
    self
  }
  pub fn set_default(&mut self, default: f64) -> &mut Self {
    self.default = Some(default);
    self
  }
}

impl ColumnTypeable for Decimal {
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
    let r#type = crate::const_data::GLOBAL_DB_KEY_MAPPING.get("decimal").unwrap();
    sql.push_value(r#type);
    sql.push_value(&format!("({}, {})", self.precision.unwrap(), self.scale.unwrap()));

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

impl From<Decimal> for ColumnType {
  fn from(decimal: Decimal) -> Self {
    ColumnType::Decimal(decimal)
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
          table.add_column_decimal("decimal_a", |_decimal| {});
          table.add_column_decimal("decimal_b", |decimal| {
            decimal.set_precision_scale(10, 5).set_default(1.0).set_comment("my comment");
          });
        }).try_into().unwrap();
        assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (
decimal_a DECIMAL(30, 6),
decimal_b DECIMAL(10, 5) DEFAULT 1 COMMENT 'my comment'
);".to_owned());

        // update table
        let sql: String = crate::SqlGen::alter_table("users", |table| {
          table.add_column_decimal("decimal_a", |_decimal| {});
          table.add_column_decimal("decimal_b", |decimal| {
            decimal.set_default(0.5).set_comment("my comment");
          });
          table.modify_column_decimal("decimal_c", |decimal| {
            decimal.set_null(false).set_index(true).set_default(0.5);
          });
          table.change_column_boolean("is_a", "is_b", |decimal| {
            decimal.set_comment("rename to is_b");
          });
          table.drop_column_boolean("decimal_d");
        }).try_into().unwrap();
        assert_eq!(sql, "ALTER TABLE users
ADD COLUMN decimal_a DECIMAL(30, 6),
ADD COLUMN decimal_b DECIMAL(30, 6) DEFAULT 0.5 COMMENT 'my comment',
MODIFY COLUMN decimal_c DECIMAL(30, 6) NOT NULL DEFAULT 0.5,
CHANGE COLUMN is_a is_b BOOLEAN COMMENT 'rename to is_b',
DROP COLUMN decimal_d;".to_owned());

      }
    }
}