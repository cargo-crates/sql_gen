
pub mod const_data;
pub mod error;
pub mod collectors;
pub mod database;
pub mod table;

pub use error::SqlError;
pub use database::{database_manager::{self, DatabaseManager}};
pub use table::{table_column::{self, TableColumn}, table_manager::{self, TableManager}};


pub struct SqlGen {
}

impl SqlGen {
    pub fn create_database(database_name: &str) -> database_manager::DatabaseManager {
        database_manager::DatabaseManager::create_database(database_name)
    }
    #[cfg(any(feature = "postgresql", feature = "mssql"))]
    pub fn rename_database(old_database_name: &str, database_name: &str) -> database_manager::DatabaseManager {
        database_manager::DatabaseManager::rename_database(old_database_name, database_name)
    }
    pub fn drop_database(database_name: &str) -> database_manager::DatabaseManager {
        database_manager::DatabaseManager::drop_database(database_name)
    }
    pub fn create_table(table_name: &str) -> table_manager::TableManager {
        table_manager::TableManager::create_table(table_name)
    }
    #[cfg(any(feature = "mysql", feature = "postgresql", feature = "mssql", feature = "sqlite"))]
    pub fn rename_table(old_table_name: &str, table_name: &str) -> table_manager::TableManager {
        table_manager::TableManager::rename_table(old_table_name, table_name)
    }
    pub fn drop_table(table_name: &str) -> table_manager::TableManager {
        table_manager::TableManager::drop_table(table_name)
    }
}