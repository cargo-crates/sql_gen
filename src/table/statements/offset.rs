use std::{marker::PhantomData};
use crate::collectors::Sql;

pub struct Offset<M: crate::Manageable> {
  pub value: usize,
  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for Offset<M> {
  fn default() -> Self {
      Self {
          value: 0,
          _marker: PhantomData
      }
  }
}

impl<M: crate::Manageable> Offset<M> {
  pub fn new(value: usize) -> Self {
    let mut offset = Offset::<M>::default();
    offset.value = value;
    offset
  }
  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::default();
    sql.push_value(&format!("OFFSET {}", self.value));

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
        let mut offset = Offset::<User>::default();
        offset.value = 10;
        let sql_string: String = offset.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "OFFSET 10");
      }
    }
}