use std::any::type_name;
use inflector::{string::{demodulize, pluralize}, cases::snakecase};
use regex::Regex;

pub fn type_to_pluralize_string<M>() -> String where M: ?Sized {
  // eg: xxx::UserTable
  let full_namespace = type_name::<M>();
  let full_namespace = Regex::new(r"<.*>$").unwrap().replace(&full_namespace, "").to_string();
  // eg: UserTable
  let struct_name = demodulize::demodulize(&full_namespace);
  // eg: user_table
  let snake_struct_name = snakecase::to_snake_case(&struct_name);
  // eg: user_tables
  // pluralize::to_plural(Regex::new(r"_arel$").unwrap().replace(&snake_struct_name, "").as_ref())
  pluralize::to_plural(&snake_struct_name)
}

pub fn json_value_to_string(value: &serde_json::Value) -> Result<String, crate::error::SqlError> {
  let string = match value {
    serde_json::Value::String(string) => format!("'{}'", string),
    serde_json::Value::Bool(boolean) => format!("{}", if *boolean {1} else {0}),
    serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
      return Err(crate::error::SqlError::Message(format!("Error: json_value_to_string value type not support! {:?}", value)));
    },
    _ => value.to_string(),
  };
  Ok(string)
}