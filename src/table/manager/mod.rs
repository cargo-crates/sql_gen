pub mod select_manager;
pub mod insert_manager;
pub mod update_manager;
pub mod delete_manager;

pub use select_manager::{SelectManager};
pub use insert_manager::{InsertManager};
pub use update_manager::{UpdateManager};
pub use delete_manager::{DeleteManager};


pub trait Manageable: Sized {
  fn table_name() -> String { crate::methods::type_to_pluralize_string::<Self>() }

  fn table_column_names() -> Vec<&'static str> { vec![] }
  fn attr_names() -> Vec<&'static str> { vec![] }
  fn attr_name_to_table_column_name<'a>(_attr_name: &'a str) -> Result<&'a str, crate::SqlError> { Err(crate::SqlError::Message("No Impl".to_owned())) }
  fn table_column_name_to_attr_name<'a>(_table_column_name: &'a str) -> Result<&'a str, crate::SqlError> { Err(crate::SqlError::Message("No Impl".to_owned())) }

  fn primary_key() -> &'static str { "id" }
  fn id() -> &'static str { Self::primary_key() }

  fn query() -> SelectManager<Self> {
    SelectManager::<Self>::default()
  }
  fn create<T: serde::Serialize>(insert_condition: T) -> InsertManager<Self> {
    let mut select_manager = InsertManager::<Self>::default();
    select_manager.insert(insert_condition);
    select_manager
  }
  fn update_all<T: serde::Serialize>(update_condition: T) -> UpdateManager<Self> {
    let mut update_manager = UpdateManager::default();
    update_manager.update(update_condition);
    update_manager
  }
  fn delete_all<T: serde::Serialize>(where_condition: T) -> DeleteManager<Self> {
    let mut delete_manager = DeleteManager::<Self>::default();
    delete_manager.r#where(where_condition);
    delete_manager
  }

  // self
  fn get_json_value_from_attr_name__(&self, _attr_name: &str) -> Option<serde_json::Value> { None }
}