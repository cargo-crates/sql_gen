use std::marker::PhantomData;
use crate::table::statements;
use crate::collectors::Sql;
pub struct UpdateManager<M: crate::Manageable> {
  update: Option<statements::Update<M>>,
  wheres: Option<Vec<statements::Where<M>>>,
  orders: Option<Vec<statements::Order<M>>>,
  limit: Option<statements::Limit<M>>,
  offset: Option<statements::Offset<M>>,

  _marker: PhantomData<M>,
}


impl<M: crate::Manageable> Default for UpdateManager<M> {
  fn default() -> Self {
      Self {
        update: None,
        wheres: None,
        orders: None,
        limit: None,
        offset: None,

        _marker: PhantomData,
      }
  }
}

impl<M: crate::Manageable> UpdateManager<M> {
  pub fn update<T: serde::Serialize>(&mut self, condition: T) -> &mut Self {
    self.update = Some(statements::Update::<M>::new(serde_json::json!(condition), None));
    self
  }
  pub fn update_prepare<T: serde::Serialize>(&mut self, condition: T) -> &mut Self {
    self.update = Some(statements::Update::<M>::new(serde_json::json!(condition), Some(true)));
    self
  }
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
    let mut sql = Sql::default();

    if let Some(ref update) = self.update {
      sql.push_sql(&update.to_sql()?);
    } else {
      return Err(crate::error::SqlError::Message("update table data update value must exist".into()))
    }

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
        let update_manager = UpdateManager::<User>::default();
        assert!(update_manager.to_sql().is_err());

        // update
        let mut update_manager = UpdateManager::<User>::default();
        update_manager.update(serde_json::json!({"a": 1, "b": true, "c": null, "d": "desc"})).r#where("a = 1");
        let sql_string: String = update_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "UPDATE users SET a = 1, b = 1, c = null, d = 'desc' WHERE a = 1");
        let mut update_manager = UpdateManager::<User>::default();
        update_manager.update_prepare(serde_json::json!({"a": 1, "b": true, "c": null, "d": "desc"})).r#where("a = 1");
        let sql_string: String = update_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "UPDATE users SET a = 1, b = 1, c = null, d = 'desc' WHERE a = 1");
      }
    }
}