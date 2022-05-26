
use once_cell::sync::Lazy;
use std::collections::HashMap;


pub static GLOBAL_KEY_MAPPING: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
  let mut m = HashMap::new();
  #[cfg(feature = "sqlite")]
  m.insert("auto_increment", "AUTOINCREMENT");
  #[cfg(not(feature = "sqlite"))]
  m.insert("auto_increment", "AUTO_INCREMENT");
  m
});