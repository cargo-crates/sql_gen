
pub mod const_data;
pub mod error;
pub mod collectors;
pub mod database;
pub mod table;

pub use const_data::GLOBAL_DB_KEY_MAPPING;
pub use error::SqlError;
pub use database::{define_database::{self, DefineDatabase}};
pub use table::{column::{self, Column, column_type::{self, ColumnType, ColumnTypeable}}, define_table::{self, DefineTable}, manager::{self}};


pub struct SqlGen {
}

impl SqlGen {
    pub fn create_database(database_name: &str) -> DefineDatabase {
        DefineDatabase::create_database(database_name)
    }
    pub fn alter_database(database_name: &str, callback: impl Fn(&mut DefineDatabase) -> ()) -> DefineDatabase {
        DefineDatabase::alter_database(database_name, callback)
    }
    pub fn drop_database(database_name: &str) -> DefineDatabase {
        DefineDatabase::drop_database(database_name)
    }
    pub fn create_table(table_name: &str, callback: impl Fn(&mut DefineTable) -> ()) -> DefineTable {
        DefineTable::create(table_name, callback)
    }
    pub fn rename_table(old_table_name: &str, new_table_name: &str) -> DefineTable {
        DefineTable::rename(old_table_name, new_table_name)
    }
    pub fn alter_table(table_name: &str, callback: impl Fn(&mut DefineTable) -> ()) -> DefineTable {
        DefineTable::alter(table_name, callback)
    }
    pub fn drop_table(table_name: &str) -> DefineTable {
        DefineTable::drop(table_name)
    }
}

pub mod prelude;