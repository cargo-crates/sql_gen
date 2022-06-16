use crate::error::SqlError;

#[derive(Clone, Debug)]
pub struct Sql {
    pub value: String,
    pub prepare_value: Option<Vec<String>>,
}

impl Default for Sql {
  fn default() -> Self {
      Self {
        value: "".to_string(),
        prepare_value: None,
      }
  }
}

impl Sql {
  pub fn new(value: String) -> Self {
      Self {
        value,
        prepare_value: None,
      }
  }
  pub fn push(&mut self, char: char) -> &mut Self {
    self.value.push(char);
    self
  }
  pub fn push_value(&mut self, sub_value: &str) -> &mut Self {
    self.value.push_str(sub_value);
    self
  }
  pub fn push_quote_value(&mut self, quote: &str, value: &str) -> &mut Self {
    self.push_value(quote);
    self.push_value(value);
    self.push_value(quote)
  }
  pub fn push_prepare_value(&mut self, sub_prepare_value: &str) -> &mut Self {
    if let Some(prepare_value) = &mut self.prepare_value {
      prepare_value.push(sub_prepare_value.into());
    } else {
        self.prepare_value = Some(vec![sub_prepare_value.into()]);
    }
    self
  }
  pub fn push_value_with_prepare_value(&mut self, sub_value: &str, sub_prepare_value: &str) -> &mut Self {
    self.value.push_str(sub_value);
    self.push_prepare_value(sub_prepare_value);
    self
  }

  pub fn push_sql(&mut self, sql: &Sql) -> &mut Self {
    if let Some(prepare_value) = &sql.prepare_value {
      if let Some(self_prepare_value) = &mut self.prepare_value {
        self_prepare_value.extend_from_slice(prepare_value)
      } else {
        self.prepare_value = Some(prepare_value.clone());
      }
      self.push_value(&sql.value);
    } else {
        self.push_value(&sql.value);
    }
    self
  }
  pub fn push_sqls(&mut self, sqls: &Vec<Sql>, join_str: &str) -> &mut Self {
    let len = sqls.len();
    for (idx, sql) in sqls.iter().enumerate() {
        self.push_sql(sql);
        if idx != len - 1 {
            self.push_value(join_str);
        }
    }
    self
  }

  pub fn to_sql_string(&self) -> Result<String, SqlError> {
    if let Some(prepare_value) = &self.prepare_value {
        let mut replace_idx = 0;
        let raw_sql = self.value.chars().map(|char|
            match char {
                '?' => {
                    let use_replace_value = prepare_value.get(replace_idx).expect("参数不足");
                    replace_idx += 1;
                    Ok(use_replace_value.to_string())
                },
                _ => Ok(char.to_string())
            }).collect::<Result<String, SqlError>>()?;
        if replace_idx == prepare_value.len() {
            Ok(raw_sql)
        } else {
            Err(SqlError::Message(format!("prepare sql params count not match: {}", raw_sql)))
        }
    } else {
        Ok(self.value.clone())
    }
  }

  pub fn is_empty(&self) -> bool {
    self.value.is_empty() && self.prepare_value.is_none()
  }
}

impl TryFrom<Sql> for String {
  type Error = crate::SqlError;
  fn try_from(sql: Sql) -> Result<String, Self::Error> {
    sql.to_sql_string()
  }
}