use std::{marker::PhantomData};
use crate::collectors::Sql;

pub struct Limit<M: crate::Manageable> {
  pub value: usize,
  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for Limit<M> {
  fn default() -> Self {
      Self {
          value: 0,
          _marker: PhantomData
      }
  }
}

impl<M: crate::Manageable> Limit<M> {
  pub fn new(value: usize) -> Self {
    let mut limit = Limit::<M>::default();
    limit.value = value;
    limit
  }
  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::default();
    sql.push_value(&format!("LIMIT {}", self.value));

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
        //
        let mut limit = Limit::<User>::default();
        limit.value = 10;
        let sql_string: String = limit.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "LIMIT 10");
      }
    }
}