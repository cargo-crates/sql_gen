use crate::collectors::Sql;
use crate::{column::column_type::{ColumnType, ColumnTypeable}};

#[derive(Clone, Debug)]
pub struct Index {
  // index_name: Option<String>,
  // BTREE | HASH
  // index_type: Option<String>,

  index: Option<bool>,
  unique: Option<bool>,
}

impl Default for Index {
  fn default() -> Index {
    Index {
      // index_name: None,
      // index_type: None,

      index: Some(true),
      unique: None,
    }
  }
}

impl Index {
}

impl ColumnTypeable for Index {
  // fn null(&self) -> Option<bool> { self.null }
  // fn set_null(&mut self, null: bool) -> &mut Self {
  //   self.null = Some(null);
  //   self
  // }

  // fn set_comment(&mut self, comment: &'static str) -> &mut Self {
  //   self.comment = Some(comment);
  //   self
  // }

  fn index(&self) -> Option<bool> { self.index }
  // fn set_index(&mut self, index: bool) -> &mut Self { self.index = Some(index); self }

  fn unique(&self) -> Option<bool> { self.unique }
  fn set_unique(&mut self, unique: bool) -> &mut Self { self.unique = Some(unique); self }

  // fn primary_key(&self) -> Option<bool> { self.primary_key }
  // fn set_primary_key(&mut self, primary_key: bool) -> &mut Self {
  //   self.primary_key = Some(primary_key);
  //   self
  // }

  // fn foreign_key(&self) -> Option<&crate::table::ForeignKey> { self.foreign_key.as_ref() }
  // fn set_foreign_key(&mut self, foreign_key: crate::table::ForeignKey) -> &mut Self {
  //   self.foreign_key = Some(foreign_key);
  //   self
  // }

  fn to_sql(&self, _column: &crate::Column, _table: &crate::Table) -> Option<Sql> {
    None
  }
}

impl From<Index> for ColumnType {
  fn from(index: Index) -> Self {
    ColumnType::Index(index)
  }
}


#[cfg(test)]

mod tests {
  use crate::prelude::*;
  #[test]
  fn to_sql() {
    #[cfg(feature = "mysql")]
    {
      // update table
      let sql: String = crate::SqlGen::alter_table("users", |table| {
        table.add_index(vec!["index_a"], |_index| {});
        table.add_index(vec!["index_b"], |index| {
          index.set_unique(true);
        });
        table.drop_index("index_on_index_c");
      }).try_into().unwrap();
      assert_eq!(sql, "ALTER TABLE users
ADD CONSTRAINT INDEX index_on_index_a (index_a)
ADD CONSTRAINT UNIQUE INDEX unique_index_on_index_b (index_b)
DROP INDEX index_on_index_c;".to_owned());

    }
  }
}