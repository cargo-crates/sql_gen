
pub mod const_data;
pub mod error;
pub mod collectors;
pub mod database;
pub mod table;

pub use const_data::GLOBAL_DB_KEY_MAPPING;
pub use error::SqlError;
pub use database::{Database};
pub use table::{column::{self, Column, column_type::{self, ColumnType, ColumnTypeable}}, Table};


pub struct SqlGen {
}

impl SqlGen {
    pub fn create_database(database_name: &str) -> Database {
        Database::create_database(database_name)
    }
    pub fn alter_database(database_name: &str, callback: impl Fn(&mut Database) -> ()) -> Database {
        Database::alter_database(database_name, callback)
    }
    pub fn drop_database(database_name: &str) -> Database {
        Database::drop_database(database_name)
    }
    pub fn create_table(table_name: &str, callback: impl Fn(&mut Table) -> ()) -> Table {
        Table::create(table_name, callback)
    }
    pub fn rename_table(old_table_name: &str, new_table_name: &str) -> Table {
        Table::rename(old_table_name, new_table_name)
    }
    pub fn alter_table(table_name: &str, callback: impl Fn(&mut Table) -> ()) -> Table {
        Table::alter(table_name, callback)
    }
    pub fn drop_table(table_name: &str) -> Table {
        Table::drop(table_name)
    }
}

pub mod prelude;