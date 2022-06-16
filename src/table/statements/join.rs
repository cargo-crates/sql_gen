use std::marker::PhantomData;
use crate::collectors::Sql;

pub struct Join<M: crate::Manageable> {
  pub value: Option<String>,
  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for Join<M> {
  fn default() -> Self {
      Self {
          value: None,
          _marker: PhantomData
      }
  }
}

impl<M: crate::Manageable> Join<M> {
  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::default();

    if let Some(ref value) = self.value {
      sql.push_value(value);
    } else {
      return Err(crate::error::SqlError::Message(format!("Error: Join Value Must Exists!")));
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
        let join = Join::<User>::default();
        assert!(join.to_sql().is_err());

        let mut join = Join::<User>::default();
        join.value = Some("left join orders on users.id = orders.user_id".into());
        let sql_string: String = join.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "left join orders on users.id = orders.user_id");
      }
    }
}