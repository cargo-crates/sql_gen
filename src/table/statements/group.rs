use std::{marker::PhantomData};
use crate::collectors::Sql;

pub struct Group<M: crate::Manageable> {
  pub value: Option<serde_json::Value>,
  _marker: PhantomData<M>,
}

impl<M: crate::Manageable> Default for Group<M> {
  fn default() -> Self {
      Self {
          value: None,
          _marker: PhantomData
      }
  }
}

impl<M: crate::Manageable> Group<M> {
  pub fn new(condition: serde_json::Value) -> Self {
    let mut group = Group::<M>::default();
    group.value = Some(condition);
    group
  }
  pub fn to_sql(&self) -> Result<Sql, crate::error::SqlError> {
    let mut sql = Sql::default();

    if let Some(ref value) = self.value {
      match value {
        serde_json::Value::Array(arr) => {
          let values: Vec<String> = arr.iter().map(|v| v.as_str().unwrap().to_string()).collect();
          eprintln!("{:?}", values);
          sql.push_value(&format!("{}", values.join(",")));
        },
        serde_json::Value::String(string) => {
          sql.push_value(string);
        },
        _ => return Err(crate::error::SqlError::Message(format!("Error: Value {:?} Type Not Support!", self.value)))
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
        let mut group = Group::<User>::default();
        group.value = Some(serde_json::json!("a"));
        let sql_string: String = group.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "a");

        let mut group = Group::<User>::default();
        group.value = Some(serde_json::json!(["a", "b", "c"]));
        let sql_string: String = group.to_sql().unwrap().try_into().unwrap();
        assert_eq!(&sql_string, "a,b,c");
      }
    }
}