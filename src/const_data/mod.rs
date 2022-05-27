
use once_cell::sync::Lazy;
use std::collections::HashMap;


pub struct DbKeyMapping {
  mapping: HashMap<&'static str, &'static str>,
}
impl Default for DbKeyMapping {
  fn default() -> Self {
    let mut m = HashMap::new();
    m.insert("auto_increment", "AUTO_INCREMENT");
    m.insert("boolean", "BOOLEAN");
    m.insert("integer", "INT");
    m.insert("float", "FLOAT");
    m.insert("double", "DOUBLE");
    m.insert("decimal", "DECIMAL");
    m.insert("string", "VARCHAR");
    m.insert("text", "TEXT");
    m.insert("time", "TIME");
    m.insert("date", "DATE");
    m.insert("datetime", "DATETIME");
    m.insert("timestamp", "TIMESTAMP");
    m.insert("json", "JSON");
    m.insert("blob", "BLOB");
    m.insert("binary", "VARBINARY");
    Self { mapping: m }
  }
}
impl DbKeyMapping {
  pub fn get(&self, key: &str) -> Option<&str> {
    match self.mapping.get(key) {
      Some(v) => Some(*v),
      None => None,
    }
  }
}

pub static GLOBAL_DB_KEY_MAPPING: Lazy<DbKeyMapping> = Lazy::new(|| DbKeyMapping::default());