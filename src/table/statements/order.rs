use std::{marker::PhantomData};
use crate::collectors::Sql;

pub struct Order<M: crate::Manageable> {
  pub value: Option<serde_json::Value>,
  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for Order<M> {
  fn default() -> Self {
      Self {
          value: None,
          _marker: PhantomData
      }
  }
}

impl<M: crate::Manageable> Order<M> {
  pub fn new(condition: serde_json::Value) -> Self {
    let mut order = Order::<M>::default();
    order.value = Some(condition);
    order
  }
  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::default();

    if let Some(ref value) = self.value {
      match value {
        serde_json::Value::Object(obj) => {
          for (_idx, column_name) in obj.keys().enumerate() {
            sql.push_value(column_name);
            let value = obj.get(column_name).unwrap();
            sql.push(' ').push_value(value.as_str().unwrap());
          }
        },
        serde_json::Value::Array(arr) => {
          let values: Vec<String> = arr.iter().map(|v| {
            match v {
              serde_json::Value::String(str) => str.into(),
              _ => v.to_string()
            }
          }).collect();
          sql.push_value(&format!("{}", values.join(",")));
        },
        serde_json::Value::String(str) => {
          sql.push_value(str);
        },
        _ => return Err(crate::error::SqlError::Message(format!("Error: Value {:?} Type Not Support", self.value)))
      }
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
        //
        let mut order = Order::<User>::default();
        order.value = Some(serde_json::json!("a"));
        let sql_string: String = order.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "a");

        let mut order = Order::<User>::default();
        order.value = Some(serde_json::json!(["a", "b", "c"]));
        let sql_string: String = order.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "a,b,c");
      }
    }
}