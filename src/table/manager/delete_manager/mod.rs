use std::marker::PhantomData;
use crate::table::statements;
use crate::collectors::Sql;
pub struct DeleteManager<M: crate::Manageable> {
  wheres: Option<Vec<statements::Where<M>>>,
  orders: Option<Vec<statements::Order<M>>>,
  limit: Option<statements::Limit<M>>,
  offset: Option<statements::Offset<M>>,

  _marker: PhantomData<M>,
}


impl<M: crate::Manageable> Default for DeleteManager<M> {
  fn default() -> Self {
      Self {
        wheres: None,
        orders: None,
        limit: None,
        offset: None,

        _marker: PhantomData,
      }
  }
}

impl<M: crate::Manageable> DeleteManager<M> {
  pub fn r#where<T: serde::Serialize>(&mut self, condition: T) -> &mut Self {
    let r#where = statements::Where::<M>::new(serde_json::json!(condition), None, None, None);
    if let Some(wheres) = &mut self.wheres {
      wheres.push(r#where);
    } else {
      self.wheres = Some(vec![r#where])
    }
    self
  }
  pub fn where_not(&mut self, condition: serde_json::Value) -> &mut Self {
    let r#where = statements::Where::<M>::new(condition, Some(true), None, None);
    if let Some(wheres) = &mut self.wheres {
      wheres.push(r#where);
    } else {
      self.wheres = Some(vec![r#where])
    }
    self
  }
  pub fn where_or(&mut self, condition: serde_json::Value) -> &mut Self {
    let r#where = statements::Where::<M>::new(condition, None, Some(true), None);
    if let Some(wheres) = &mut self.wheres {
      wheres.push(r#where);
    } else {
      self.wheres = Some(vec![r#where])
    }
    self
  }
  pub fn where_not_or(&mut self, condition: serde_json::Value) -> &mut Self {
    let r#where = statements::Where::<M>::new(condition, Some(true), Some(true), None);
    if let Some(wheres) = &mut self.wheres {
      wheres.push(r#where);
    } else {
      self.wheres = Some(vec![r#where])
    }
    self
  }
  pub fn where_prepare(&mut self, condition: serde_json::Value) -> &mut Self {
    let r#where = statements::Where::<M>::new(condition, None, None, Some(true));
    if let Some(wheres) = &mut self.wheres {
      wheres.push(r#where);
    } else {
      self.wheres = Some(vec![r#where])
    }
    self
  }
  pub fn where_not_prepare(&mut self, condition: serde_json::Value) -> &mut Self {
    let r#where = statements::Where::<M>::new(condition, Some(true), None, Some(true));
    if let Some(wheres) = &mut self.wheres {
      wheres.push(r#where);
    } else {
      self.wheres = Some(vec![r#where])
    }
    self
  }
  pub fn where_or_prepare(&mut self, condition: serde_json::Value) -> &mut Self {
    let r#where = statements::Where::<M>::new(condition, None, Some(true), Some(true));
    if let Some(wheres) = &mut self.wheres {
      wheres.push(r#where);
    } else {
      self.wheres = Some(vec![r#where])
    }
    self
  }
  pub fn where_not_or_prepare(&mut self, condition: serde_json::Value) -> &mut Self {
    let r#where = statements::Where::<M>::new(condition, Some(true), Some(true), Some(true));
    if let Some(wheres) = &mut self.wheres {
      wheres.push(r#where);
    } else {
      self.wheres = Some(vec![r#where])
    }
    self
  }
  pub fn where_range<T: ToString>(&mut self, column_name: &str, range: impl std::ops::RangeBounds<T>) -> &mut Self {
    let r#where = statements::Where::<M>::new_range(column_name, range);
    if let Some(wheres) = &mut self.wheres {
      wheres.push(r#where);
    } else {
      self.wheres = Some(vec![r#where])
    }
    self
  }
  pub fn order<T: serde::Serialize>(&mut self, condition: T) -> &mut Self {
    let order = statements::Order::<M>::new(serde_json::json!(condition));
    if let Some(orders) = &mut self.orders {
      orders.push(order);
    } else {
      self.orders = Some(vec![order])
    }
    self
  }
  pub fn limit(&mut self, value: usize) -> &mut Self {
    self.limit = Some(statements::Limit::new(value));
    self
  }
  pub fn offset(&mut self, value: usize) -> &mut Self {
    self.offset = Some(statements::Offset::new(value));
    self
  }
  pub fn paginate(&mut self, page: usize, page_size: usize) -> &mut Self {
    let offset = (page - 1) * page_size;
    self.limit(page_size);
    self.offset(offset);
    self
}
  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::new(format!("DELETE FROM {}", M::table_name()));

    if let Some(ref wheres) = self.wheres {
      sql.push_value(" WHERE");
      for (idx, r#where) in wheres.iter().enumerate() {
        if idx > 0 {
          sql.push_value(" AND");
        }
        if let Some(or) = r#where.or && or {
          sql.push_value(" (").push_sql(&r#where.to_sql()?).push(')');
        } else {
          sql.push(' ').push_sql(&r#where.to_sql()?);
        }
      }
    } else {
      return Err(crate::error::SqlError::Message("delete table data where statements must exist".into()))
    }

    if let Some(ref orders) = self.orders {
      sql.push_value(" ORDER BY ");
      for (idx, order) in orders.iter().enumerate() {
        if idx > 0 {
          sql.push(',');
        }
        sql.push_sql(&order.to_sql()?);
      }
    }

    if let Some(ref limit) = self.limit {
      sql.push(' ').push_sql(&limit.to_sql()?);
    }

    if let Some(ref offset) = self.offset {
      sql.push(' ').push_sql(&offset.to_sql()?);
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
        let delete_manager = DeleteManager::<User>::default();
        assert!(delete_manager.to_sql().is_err());

        // where
        let mut delete_manager = DeleteManager::<User>::default();
        delete_manager.r#where("a = 1");
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1");
        let mut delete_manager = DeleteManager::<User>::default();
        delete_manager.r#where(serde_json::json!("a = 1"));
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1");

        let mut delete_manager = DeleteManager::<User>::default();
        delete_manager.r#where(serde_json::json!(["a = ?", 1]));
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1");
        delete_manager.where_not(serde_json::json!({"b": [1, 2, 3]}));
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1 AND b NOT IN (1,2,3)");
        delete_manager.where_or(serde_json::json!({"c1": true, "c2": false}));
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1 AND b NOT IN (1,2,3) AND (c1 = 1 OR c2 = 0)");
        delete_manager.where_not_prepare(serde_json::json!({"d": [1, 2, 3]}));
        let sql = delete_manager.to_sql().unwrap();
        assert_eq!(&sql.value, "DELETE FROM users WHERE a = ? AND b NOT IN (1,2,3) AND (c1 = 1 OR c2 = 0) AND d NOT IN (?,?,?)");
        let sql_string: String = sql.try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1 AND b NOT IN (1,2,3) AND (c1 = 1 OR c2 = 0) AND d NOT IN (1,2,3)");

        let mut delete_manager = DeleteManager::<User>::default();
        delete_manager.where_range("id", 1..100);
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE id >= 1 AND id < 100");
        let mut delete_manager = DeleteManager::<User>::default();
        delete_manager.where_range("id", 1..=100);
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE id BETWEEN 1 AND 100");

        // order
        let mut delete_manager = DeleteManager::<User>::default();
        delete_manager.order("id desc");
        delete_manager.order(serde_json::json!({"created_at": "desc"}));
        delete_manager.order(vec!["updated_at desc"]);
        assert!(delete_manager.to_sql().is_err());
        delete_manager.where_not_or(serde_json::json!({"a": 1, "b": [1, 2, 3]}));
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE (a != 1 OR b NOT IN (1,2,3)) ORDER BY id desc,created_at desc,updated_at desc");

        // limit
        let mut delete_manager = DeleteManager::<User>::default();
        delete_manager.limit(10);
        delete_manager.r#where("a = 1");
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1 LIMIT 10");

        // offset
        let mut delete_manager = DeleteManager::<User>::default();
        delete_manager.offset(5);
        delete_manager.r#where("a = 1");
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1 OFFSET 5");

        // paginate
        let mut delete_manager = DeleteManager::<User>::default();
        delete_manager.r#where("a = 1");
        delete_manager.paginate(1, 20);
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1 LIMIT 20 OFFSET 0");
        delete_manager.paginate(2, 20);
        let sql_string: String = delete_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "DELETE FROM users WHERE a = 1 LIMIT 20 OFFSET 20");
      }
    }
}