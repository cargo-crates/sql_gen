use std::marker::PhantomData;
use crate::table::statements;
use crate::collectors::Sql;

pub struct SelectManager<M: crate::Manageable> {
  select: statements::Select<M>,
  joins: Option<Vec<statements::Join<M>>>,
  wheres: Option<Vec<statements::Where<M>>>,
  groups: Option<Vec<statements::Group<M>>>,
  havings: Option<Vec<statements::Having<M>>>,
  orders: Option<Vec<statements::Order<M>>>,
  limit: Option<statements::Limit<M>>,
  offset: Option<statements::Offset<M>>,

  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for SelectManager<M> {
  fn default() -> Self {
      Self {
        select: statements::Select::<M>::default(),
        joins: None,
        wheres: None,
        groups: None,
        havings: None,
        orders: None,
        limit: None,
        offset: None,

        _marker: PhantomData,
      }
  }
}

impl<M: crate::Manageable> SelectManager<M> {
  pub fn distinct(&mut self) -> &mut Self {
    self.select.distinct = Some(true);
    self
  }
  pub fn joins(&mut self, condition: &str) -> &mut Self {
    let mut join = statements::Join::<M>::default();
    join.value = Some(condition.into());
    if let Some(joins) = &mut self.joins {
      joins.push(join);
    } else {
      self.joins = Some(vec![join])
    }
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
  pub fn group<T: serde::Serialize>(&mut self, condition: T) -> &mut Self {
    let group = statements::Group::<M>::new(serde_json::json!(condition));
    if let Some(groups) = &mut self.groups {
      groups.push(group);
    } else {
      self.groups = Some(vec![group])
    }
    self
  }
  pub fn having<T: serde::Serialize>(&mut self, condition: T) -> &mut Self {
    let having = statements::Having::<M>::new(serde_json::json!(condition), None, None, None);
    if let Some(havings) = &mut self.havings {
      havings.push(having);
    } else {
      self.havings = Some(vec![having])
    }
    self
  }
  pub fn having_not(&mut self, condition: serde_json::Value) -> &mut Self {
    let having = statements::Having::<M>::new(condition, Some(true), None, None);
    if let Some(havings) = &mut self.havings {
      havings.push(having);
    } else {
      self.havings = Some(vec![having])
    }
    self
  }
  pub fn having_or(&mut self, condition: serde_json::Value) -> &mut Self {
    let having = statements::Having::<M>::new(condition, None, Some(true), None);
    if let Some(havings) = &mut self.havings {
      havings.push(having);
    } else {
      self.havings = Some(vec![having])
    }
    self
  }
  pub fn having_not_or(&mut self, condition: serde_json::Value) -> &mut Self {
    let having = statements::Having::<M>::new(condition, Some(true), Some(true), None);
    if let Some(havings) = &mut self.havings {
      havings.push(having);
    } else {
      self.havings = Some(vec![having])
    }
    self
  }
  pub fn having_prepare(&mut self, condition: serde_json::Value) -> &mut Self {
    let having = statements::Having::<M>::new(condition, None, None, Some(true));
    if let Some(havings) = &mut self.havings {
      havings.push(having);
    } else {
      self.havings = Some(vec![having])
    }
    self
  }
  pub fn having_not_prepare(&mut self, condition: serde_json::Value) -> &mut Self {
    let having = statements::Having::<M>::new(condition, Some(true), None, Some(true));
    if let Some(havings) = &mut self.havings {
      havings.push(having);
    } else {
      self.havings = Some(vec![having])
    }
    self
  }
  pub fn having_or_prepare(&mut self, condition: serde_json::Value) -> &mut Self {
    let having = statements::Having::<M>::new(condition, None, Some(true), Some(true));
    if let Some(havings) = &mut self.havings {
      havings.push(having);
    } else {
      self.havings = Some(vec![having])
    }
    self
  }
  pub fn having_not_or_prepare(&mut self, condition: serde_json::Value) -> &mut Self {
    let having = statements::Having::<M>::new(condition, Some(true), Some(true), Some(true));
    if let Some(havings) = &mut self.havings {
      havings.push(having);
    } else {
      self.havings = Some(vec![having])
    }
    self
  }
  pub fn having_range<T: ToString>(&mut self, column_name: &str, range: impl std::ops::RangeBounds<T>) -> &mut Self {
    let having = statements::Having::<M>::new_range(column_name, range);
    if let Some(havings) = &mut self.havings {
      havings.push(having);
    } else {
      self.havings = Some(vec![having])
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
    sql.push_sql(&self.select.to_sql()?);

    if let Some(ref joins) = self.joins {
      for join in joins.iter() {
        sql.push(' ').push_sql(&join.to_sql()?);
      }
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
    }

    if let Some(ref groups) = self.groups {
      sql.push_value(" GROUP BY ");
      for (idx, group) in groups.iter().enumerate() {
        if idx > 0 {
          sql.push(',');
        }
        sql.push_sql(&group.to_sql()?);
      }
    }

    if let Some(ref havings) = self.havings {
      sql.push_value(" HAVING");
      for (idx, having) in havings.iter().enumerate() {
        if idx > 0 {
          sql.push_value(" AND");
        }
        if let Some(or) = having.or && or {
          sql.push_value(" (").push_sql(&having.to_sql()?).push(')');
        } else {
          sql.push(' ').push_sql(&having.to_sql()?);
        }
      }
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
        let mut select_manager = SelectManager::<User>::default();
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users");
        // distinct
        select_manager.distinct();
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT DISTINCT * FROM users");

        // join
        let mut select_manager = SelectManager::<User>::default();
        select_manager.joins("left join orders on users.id = orders.user_id");
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users left join orders on users.id = orders.user_id");
        select_manager.joins("left join walltes on users.id = wallets.user_id");
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users left join orders on users.id = orders.user_id left join walltes on users.id = wallets.user_id");

        // where
        let mut select_manager = SelectManager::<User>::default();
        select_manager.r#where("a = 1");
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users WHERE a = 1");
        let mut select_manager = SelectManager::<User>::default();
        select_manager.r#where(serde_json::json!("a = 1"));
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users WHERE a = 1");

        let mut select_manager = SelectManager::<User>::default();
        select_manager.r#where(serde_json::json!(["a = ?", "1"]));
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users WHERE a = '1'");
        select_manager.where_not(serde_json::json!({"b": [1, 2, 3]}));
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users WHERE a = '1' AND b NOT IN (1,2,3)");
        select_manager.where_or(serde_json::json!({"c1": true, "c2": false}));
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users WHERE a = '1' AND b NOT IN (1,2,3) AND (c1 = 1 OR c2 = 0)");
        select_manager.where_not_prepare(serde_json::json!({"d": [1, 2, 3]}));
        let sql = select_manager.to_sql().unwrap();
        assert_eq!(&sql.value, "SELECT * FROM users WHERE a = ? AND b NOT IN (1,2,3) AND (c1 = 1 OR c2 = 0) AND d NOT IN (?,?,?)");
        let sql_string: String = sql.try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users WHERE a = '1' AND b NOT IN (1,2,3) AND (c1 = 1 OR c2 = 0) AND d NOT IN (1,2,3)");

        let mut select_manager = SelectManager::<User>::default();
        select_manager.where_range("id", 1..100);
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users WHERE id >= 1 AND id < 100");
        let mut select_manager = SelectManager::<User>::default();
        select_manager.where_range("id", 1..=100);
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users WHERE id BETWEEN 1 AND 100");

        // group
        let mut select_manager = SelectManager::<User>::default();
        select_manager.group("age");
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users GROUP BY age");
        select_manager.group(vec!["name", "province"]);
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users GROUP BY age,name,province");
        // having
        select_manager.having(serde_json::json!({"a": 1, "b": 2}));
        select_manager.having_range("c", 1..100);
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users GROUP BY age,name,province HAVING a = 1 AND b = 2 AND c >= 1 AND c < 100");

        // order
        let mut select_manager = SelectManager::<User>::default();
        select_manager.order("id desc");
        select_manager.order(serde_json::json!({"created": "desc"}));
        select_manager.order(vec!["updated_at desc"]);
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users ORDER BY id desc,created desc,updated_at desc");

        // limit
        let mut select_manager = SelectManager::<User>::default();
        select_manager.limit(10);
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users LIMIT 10");

        // offset
        let mut select_manager = SelectManager::<User>::default();
        select_manager.offset(5);
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users OFFSET 5");

        // paginate
        let mut select_manager = SelectManager::<User>::default();
        select_manager.paginate(1, 20);
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users LIMIT 20 OFFSET 0");
        select_manager.paginate(2, 20);
        let sql_string: String = select_manager.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "SELECT * FROM users LIMIT 20 OFFSET 20");
      }
    }
}