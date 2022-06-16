use std::marker::PhantomData;
use crate::collectors::Sql;
use std::ops::{Bound, RangeBounds};

pub struct Where<M: crate::Manageable> {
  pub value: Option<serde_json::Value>,
  pub not: Option<bool>,
  pub or: Option<bool>,
  pub prepare: Option<bool>,
  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for Where<M> {
  fn default() -> Self {
      Self {
        value: None,
        not: None,
        or: None,
        prepare: None,
        _marker: PhantomData
      }
  }
}

impl<M: crate::Manageable> Where<M> {
  pub fn new(condition: serde_json::Value, not: Option<bool>, or: Option<bool>, prepare: Option<bool>) -> Self {
    let mut r#where = Where::<M>::default();
    r#where.value = Some(condition);
    r#where.not = not;
    r#where.or = or;
    r#where.prepare = prepare;
    r#where
  }
  pub fn new_range<T: ToString>(column_name: &str, range: impl RangeBounds<T>) -> Self {
    let generate_raw_sql = || -> Result<String, crate::error::SqlError> {
      let raw_sql;

      let get_bound_value = |value: &T| {
        value.to_string()
      };

      match range.start_bound() {
          Bound::Unbounded => {
              match range.end_bound() {
                  Bound::Unbounded => return Err(crate::error::SqlError::Message(format!("Error: Not Support"))),
                  Bound::Included(end) => raw_sql = format!("{} <= {}", column_name, get_bound_value(end)),
                  Bound::Excluded(end) => raw_sql = format!("{} < {}", column_name, get_bound_value(end)),
              }
          },
          Bound::Included(start) => {
              match range.end_bound() {
                  Bound::Unbounded => {
                      raw_sql = format!("{} >= {}", column_name, get_bound_value(start))
                  },
                  Bound::Included(end) => raw_sql = format!("{} BETWEEN {} AND {}", column_name, get_bound_value(start), get_bound_value(end)),
                  Bound::Excluded(end) => raw_sql = format!("{} >= {} AND {} < {}", column_name, get_bound_value(start), column_name, get_bound_value(end)),
              }
          },
          Bound::Excluded(start) => {
              match range.end_bound() {
                  Bound::Unbounded => raw_sql = format!("{} > {}", column_name, get_bound_value(start)),
                  Bound::Included(end) => raw_sql = format!("{} > {} AND {} <= {}", column_name, get_bound_value(start), column_name, get_bound_value(end)),
                  Bound::Excluded(end) => raw_sql = format!("{} > {} AND {} < {}", column_name, get_bound_value(start), column_name, get_bound_value(end)),
              }
          },
      }
      Ok(raw_sql)
    };

    let raw_sql = generate_raw_sql().unwrap();
    Where::<M>::new(serde_json::json!(raw_sql), None, None, None)
  }
  fn column_value_condition_to_sql(&self, column_value_condition: &serde_json::Value) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::default();
    match column_value_condition {
      serde_json::Value::Array(arr) => {
        let values = arr.iter().map(|v| crate::methods::json_value_to_string(v)).collect::<Result<Vec<String>, crate::SqlError>>()?;
        if let Some(prepare) = self.prepare && prepare {
          if let Some(not) = self.not && not {
            sql.push_value("NOT IN (");
          } else {
            sql.push_value("IN (");
          }
          let len = values.len();
          for (idx, value) in values.iter().enumerate() {
            if idx < len - 1 {
              sql.push_value_with_prepare_value("?,", value);
            } else {
              sql.push_value_with_prepare_value("?", value);
            }
          }
          sql.push(')');
        } else {
          if let Some(not) = self.not && not {
            sql.push_value(&format!("NOT IN ({})", values.join(",")));
          } else {
            sql.push_value(&format!("IN ({})", values.join(",")));
          }
        }
      },
      serde_json::Value::String(_) | serde_json::Value::Number(_) | serde_json::Value::Bool(_) => {
        let value = crate::methods::json_value_to_string(column_value_condition)?;
        if let Some(prepare) = self.prepare && prepare {
          if let Some(not) = self.not && not {
            sql.push_value_with_prepare_value("!= ?", &value);
          } else {
            sql.push_value_with_prepare_value("= ?", &value);
          }
        } else {
          if let Some(not) = self.not && not {
            sql.push_value(&format!("!= {}", value));
          } else {
            sql.push_value(&format!("= {}", value));
          }
        }
      },
      serde_json::Value::Null => {
        if let Some(not) = self.not && not {
          sql.push_value(&format!("IS NOT NULL"));
        } else {
          sql.push_value(&format!("IS NULL"));
        }
      },
      _ => return Err(crate::error::SqlError::Message(format!("Error: Where Value {:?} Type Not Support!", column_value_condition)))
    }
    Ok(sql)
  }
  fn condition_to_sql(&self, condition: &serde_json::Value) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::default();
    match condition {
      serde_json::Value::Object(obj) => {
        for (idx, column_name) in obj.keys().enumerate() {
          if idx > 0 {
            if let Some(or) = self.or && or {
              sql.push_value(" OR ");
            } else {
              sql.push_value(" AND ");
            }
          }
          sql.push_value(column_name);
          let column_value = obj.get(column_name).unwrap();

          let value_sql = self.column_value_condition_to_sql(column_value)?;
          sql.push(' ').push_sql(&value_sql);
        }
      },
      serde_json::Value::Array(arr) => {
        for (idx, value) in arr.iter().enumerate() {
          if idx == 0 {
            sql.push_value(value.as_str().unwrap());
          } else {
            let value = crate::methods::json_value_to_string(value)?;
            sql.push_prepare_value(&value);
          }
        }
      },
      serde_json::Value::String(string) => {
        sql.push_value(string);
      }
      _ => ()
    }
    Ok(sql)
  }


  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let sql;

    if let Some(ref condition) = self.value {
      sql = self.condition_to_sql(condition)?;
    } else {
      return Err(crate::error::SqlError::Message(format!("where value must exists!")));
    }

    Ok(sql)
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone};
    #[test]
    fn to_sql() {
      struct User {}
      impl crate::Manageable for User {}

      #[cfg(feature = "mysql")]
      {
        //
        let r#where = Where::<User>::default();
        assert!(r#where.to_sql().is_err());

        let mut r#where = Where::<User>::default();
        r#where.value = Some(serde_json::json!({"a": 1, "b": "2", "c": true, "d": [1, 2, 3], "e": null}));
        let sql_string: String = r#where.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "a = 1 AND b = '2' AND c = 1 AND d IN (1,2,3) AND e IS NULL");

        r#where.not = Some(true);
        let sql_string: String = r#where.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "a != 1 AND b != '2' AND c != 1 AND d NOT IN (1,2,3) AND e IS NOT NULL");

        r#where.prepare = Some(true);
        let sql = r#where.to_sql().unwrap();
        assert_eq!(&sql.value, "a != ? AND b != ? AND c != ? AND d NOT IN (?,?,?) AND e IS NOT NULL");

        //
        let mut r#where = Where::<User>::default();
        r#where.value = Some(serde_json::json!(["active = ?", true]));
        let sql_string: String = r#where.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "active = 1");

        r#where.prepare = Some(true);
        let sql = r#where.to_sql().unwrap();
        assert_eq!(&sql.value, "active = ?");

        //
        let r#where = Where::<User>::new_range("id", 1..100);
        let sql_string: String = r#where.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "id >= 1 AND id < 100");
        let r#where = Where::<User>::new_range("id", 1..=100);
        let sql_string: String = r#where.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "id BETWEEN 1 AND 100");
        let r#where = Where::<User>::new_range("expired_at", ..=chrono::Utc.ymd(2021, 12, 31).and_hms(23, 59, 59));
        let sql_string: String = r#where.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "expired_at <= 2021-12-31 23:59:59 UTC");
        let r#where = Where::<User>::new_range("expired_at", chrono::Utc.ymd(2021, 12, 31).and_hms(23, 59, 59)..);
        let sql_string: String = r#where.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "expired_at >= 2021-12-31 23:59:59 UTC");
      }
    }
}