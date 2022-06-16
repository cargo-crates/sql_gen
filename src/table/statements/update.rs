use std::{marker::PhantomData};
use crate::collectors::Sql;

pub struct Update<M: crate::Manageable> {
  pub value: Option<serde_json::Value>,
  pub prepare: Option<bool>,
  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for Update<M> {
  fn default() -> Self {
      Self {
          value: None,
          prepare: None,
          _marker: PhantomData
      }
  }
}

impl<M: crate::Manageable> Update<M> {
  pub fn new(condition: serde_json::Value, prepare: Option<bool>) -> Self {
    let mut update = Update::default();
    update.value = Some(condition);
    update.prepare = prepare;
    update
  }
  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::new(format!("UPDATE {} SET", M::table_name()));

    if let Some(ref value) = self.value {
      match value {
        serde_json::Value::Object(obj) => {
          let column_names: Vec<String> = obj.keys().into_iter().map(|column_name| column_name.into()).collect();
          let column_values = obj.keys().into_iter().map(|column_name| crate::methods::json_value_to_string(obj.get(column_name).unwrap())).collect::<Result<Vec<String>, crate::SqlError>>()?;

          sql.push_value(" ");
          for (idx, column_name) in column_names.iter().enumerate() {
            if idx > 0 { sql.push_value(", "); }
            sql.push_value(&format!("{} = ", column_name));
            if let Some(prepare) = self.prepare && prepare {
              sql.push_value_with_prepare_value("?", &column_values[idx]);
            } else {
              sql.push_value(&column_values[idx]);
            }
          }
        },
        _ => return Err(crate::error::SqlError::Message(format!("Error: Update Value {:?} Not Support", value)))
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
        let mut update = Update::<User>::default();
        update.value = Some(serde_json::json!({
          "a": 1,
          "b": false,
          "c": null,
          "d": "desc"
        }));
        let sql_string: String = update.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "UPDATE users SET a = 1, b = 0, c = null, d = 'desc'");

        update.prepare = Some(true);
        let sql = update.to_sql().unwrap();
        assert_eq!(&sql.value, "UPDATE users SET a = ?, b = ?, c = ?, d = ?");
      }
    }
}