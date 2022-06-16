use std::marker::PhantomData;
use crate::collectors::Sql;

pub struct Select<M: crate::Manageable> {
  pub columns: Option<Vec<String>>,
  pub distinct: Option<bool>,
  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for Select<M> {
  fn default() -> Self {
      Self {
          columns: None,
          distinct: None,
          _marker: PhantomData
      }
  }
}

impl<M: crate::Manageable> Select<M> {
  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::default();
    sql.push_value("SELECT ");

    if let Some(distinct) = self.distinct && distinct {
      sql.push_value("DISTINCT ");
    }

    match self.columns {
      Some(ref columns) => {
        let value = columns.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",");
        sql.push_value(&value).push(' ');
      },
      None => {
        sql.push_value("* ");
      }
    }
    sql.push_value(&format!("FROM {}", M::table_name()));

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
        let select = Select::<User>::default();
        let sql_string: String = select.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users");

        let mut select = Select::<User>::default();
        select.columns = Some(vec!["id".into(), "name".into()]);
        select.distinct = Some(true);
        let sql_string: String = select.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT DISTINCT id,name FROM users");
      }
    }
}