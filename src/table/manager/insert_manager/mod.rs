use std::marker::PhantomData;
use crate::table::statements;
use crate::collectors::Sql;

pub struct InsertManager<M: crate::Manageable> {
  insert: Option<statements::Insert<M>>,
  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for InsertManager<M> {
  fn default() -> Self {
      Self {
        insert: None,
        _marker: PhantomData,
      }
  }
}

impl<M: crate::Manageable> InsertManager<M> {
  pub fn insert<T: serde::Serialize>(&mut self, condition: T) -> &mut Self {
    self.insert = Some(statements::Insert::<M>::new(serde_json::json!(condition), None));
    self
  }
  pub fn insert_prepare<T: serde::Serialize>(&mut self, condition: T) -> &mut Self {
    self.insert = Some(statements::Insert::<M>::new(serde_json::json!(condition), Some(true)));
    self
  }
  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::default();

    if let Some(ref insert) = self.insert {
      sql.push_sql(&insert.to_sql()?);
    }  else {
      return Err(crate::error::SqlError::Message("insert table data insert value must exist".into()))
    }

    Ok(sql)
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_sql() {
      struct User {}
      impl crate::Manageable for User {}

      #[cfg(feature = "mysql")]
      {
        let mut insert_manager = InsertManager::<User>::default();
        assert!(insert_manager.to_sql().is_err());
        insert_manager.insert(serde_json::json!({"a": 1, "b": true, "c": null, "d": "desc"}));
        let sql_string: String = insert_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "INSERT INTO users (a, b, c, d) VALUES (1, 1, null, 'desc')");
      }
    }
}