use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct Json {
  default: Option<String>,
  null: Option<bool>,
  comment: Option<&'static str>,
  index: Option<bool>,
  unique: Option<bool>,
  primary_key: Option<bool>,
}

impl Default for Json {
  fn default() -> Json {
    Json {
      default: None,
      null: None,
      comment: None,
      index: None,
      unique: None,
      primary_key: None,
    }
  }
}

impl Json {
    pub fn set_default(&mut self, default: &str) -> &mut Self {
      self.default = Some(default.to_string());
      self
    }
}

impl ColumnTypeable for Json {
  fn null(&self) -> Option<bool> { self.null }
  fn set_null(&mut self, null: bool) -> &mut Self { self.null = Some(null); self }

  fn set_comment(&mut self, comment: &'static str) -> &mut Self { self.comment = Some(comment); self }

  fn index(&self) -> Option<bool> { self.index }
  fn set_index(&mut self, index: bool) -> &mut Self { self.index = Some(index); self }

  fn unique(&self) -> Option<bool> { self.unique }
  fn set_unique(&mut self, unique: bool) -> &mut Self { self.unique = Some(unique); self }

  fn primary_key(&self) -> Option<bool> { self.primary_key }
  fn set_primary_key(&mut self, primary_key: bool) -> &mut Self { self.primary_key = Some(primary_key); self }

  fn to_sql(&self, _column: &crate::Column, _table: &crate::DefineTable) -> Option<Sql> {
    let mut sql = Sql::default();
    sql.push_value(crate::GLOBAL_DB_KEY_MAPPING.get("json").unwrap());
    if let Some(null) = self.null {
      sql.push_value(&format!(" {}", if null { "NULL" } else { "NOT NULL" }));
    }
    if let Some(ref default) = self.default {
      sql.push_value(&format!(" DEFAULT ({})", default));
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


impl From<Json> for ColumnType {
  fn from(json: Json) -> Self {
    ColumnType::Json(json)
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
          table.add_column_json("json_a", |_json| {});
          table.add_column_json("json_b", |json| {
            json.set_default("{}").set_comment("my comment");
          });
        }).try_into().unwrap();
        assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (
json_a JSON,
json_b JSON DEFAULT ({}) COMMENT 'my comment'
);".to_owned());

        // update table
        let sql: String = crate::SqlGen::alter_table("users", |table| {
          table.add_column_json("json_a", |_json| {});
          table.add_column_json("json_b", |json| {
            json.set_default("{}").set_comment("my comment");
          });
          table.modify_column_json("json_c", |json| {
            json.set_null(false).set_index(true);
          });
          table.change_column_json("is_a", "is_b", |json| {
            json.set_comment("rename to is_b");
          });
          table.drop_column_json("json_d");
        }).try_into().unwrap();
        assert_eq!(sql, "ALTER TABLE users
ADD COLUMN json_a JSON,
ADD COLUMN json_b JSON DEFAULT ({}) COMMENT 'my comment',
MODIFY COLUMN json_c JSON NOT NULL,
CHANGE COLUMN is_a is_b JSON COMMENT 'rename to is_b',
DROP COLUMN json_d;".to_owned());

      }
    }
}