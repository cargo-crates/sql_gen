use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct ForeignKey {
  foreign_key: Option<crate::define_table::ForeignKey>,
}

impl Default for ForeignKey {
  fn default() -> ForeignKey {
    ForeignKey {
      foreign_key: None,
    }
  }
}

impl ForeignKey {
}

impl ColumnTypeable for ForeignKey {
  // fn null(&self) -> Option<bool> { self.null }
  // fn set_null(&mut self, null: bool) -> &mut Self {
  //   self.null = Some(null);
  //   self
  // }

  // fn set_comment(&mut self, comment: &'static str) -> &mut Self {
  //   self.comment = Some(comment);
  //   self
  // }

  // fn index(&self) -> Option<bool> { self.index }
  // fn set_index(&mut self, index: bool) -> &mut Self { self.index = Some(index); self }

  // fn unique(&self) -> Option<bool> { self.unique }
  // fn set_unique(&mut self, unique: bool) -> &mut Self { self.unique = Some(unique); self }

  // fn primary_key(&self) -> Option<bool> { self.primary_key }
  // fn set_primary_key(&mut self, primary_key: bool) -> &mut Self {
  //   self.primary_key = Some(primary_key);
  //   self
  // }

  fn foreign_key(&self) -> Option<&crate::define_table::ForeignKey> { self.foreign_key.as_ref() }
  fn set_foreign_key(&mut self, foreign_key: crate::define_table::ForeignKey) -> &mut Self {
    self.foreign_key = Some(foreign_key);
    self
  }

  fn to_sql(&self, _column: &crate::Column, _table: &crate::DefineTable) -> Option<Sql> {
    None
  }
}

impl From<ForeignKey> for ColumnType {
  fn from(foreign_key: ForeignKey) -> Self {
    ColumnType::ForeignKey(foreign_key)
  }
}


#[cfg(test)]

mod tests {
  // use crate::prelude::*;
    #[test]
    fn to_sql() {
      #[cfg(feature = "mysql")]
      {
        // update table
        let sql: String = crate::SqlGen::alter_table("users", |table| {
          table.add_foreign_key("user_id", |_foreign_key| {});
        }).try_into().unwrap();
        assert_eq!(sql, "ALTER TABLE users
ADD CONSTRAINT FOREIGN KEY fk_on_user_id (user_id) REFERENCES users (id);".to_owned());

      }
    }
}