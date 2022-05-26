use crate::table::table_column::{TableColumn};
use crate::collectors::Sql;

enum Action {
  Create,
  #[cfg(any(feature = "mysql", feature = "postgresql", feature = "mssql", feature = "sqlite"))]
  Rename(String),
  Drop,
}

pub struct TableManager {
  action: Action,
  pub table_name: String,
  pub charset: String,
  pub collation: String,
  // foreign key restrict
  pub force: String,
  pub columns: Vec<TableColumn>,
}

impl Default for TableManager {
  fn default() -> TableManager {
    TableManager {
      action: Action::Create,
      table_name: "".to_owned(),
      charset: "utf8mb4".to_owned(),
      collation: "utf8mb4_0900_ai_ci".to_owned(),
      force: "ON UPDATE CASCADE".to_owned(),
      columns: vec![],
    }
  }
}

impl TableManager {
  pub fn create_table(table_name: &str) -> Self {
    let mut table_manager = TableManager::default();
    table_manager.action = Action::Create;
    table_manager.table_name = table_name.to_owned();
    table_manager
  }
  #[cfg(any(feature = "mysql", feature = "postgresql", feature = "mssql", feature = "sqlite"))]
  pub fn rename_table(old_table_name: &str, table_name: &str) -> Self {
    let mut table_manager = TableManager::default();
    table_manager.action = Action::Rename(old_table_name.to_owned());
    table_manager.table_name = table_name.to_owned();
    table_manager
  }
  pub fn drop_table(table_name: &str) -> Self {
    let mut table_manager = TableManager::default();
    table_manager.action = Action::Drop;
    table_manager.table_name = table_name.to_owned();
    table_manager
  }
  pub fn to_sql(&self) -> Sql {
    match &self.action {
      Action::Create => {
        let mut sql = Sql::new(format!("CREATE TABLE IF NOT EXISTS {}", self.table_name));
        sql.push(' ').push('(').push('\n');
        {
          sql.push_value(&format!("id INTEGER NOT NULL {} PRIMARY KEY", crate::const_data::GLOBAL_KEY_MAPPING.get("auto_increment").unwrap()));
          for column in self.columns.iter() {
            sql.push(',').push('\n').push_sql(&column.to_sql());
          }
        }
        sql.push('\n').push(')');
        #[cfg(not(feature = "sqlite"))]
        sql.push_value(&format!(" CHARACTER SET {} COLLATE {}", self.charset, self.collation));
        sql.push(';');
        sql
      },
      #[cfg(any(feature = "mysql", feature = "postgresql", feature = "mssql", feature = "sqlite"))]
      Action::Rename(old_table_name) => {
        #[cfg(feature = "mysql")]
        {
          Sql::new(format!("ALTER TABLE {} RENAME {};", old_table_name, self.table_name))
        }
        #[cfg(any(feature = "postgresql", feature = "sqlite"))]
        {
          Sql::new(format!("ALTER TABLE {} RENAME TO {};", old_table_name, self.table_name))
        }
        #[cfg(feature = "mssql")]
        {
          Sql::new(format!("sp_rename {},{};", old_table_name, self.table_name))
        }
      },
      Action::Drop => {
        Sql::new(format!("DROP TABLE IF EXISTS {};", self.table_name))
      }
    }
  }
}

impl TryFrom<TableManager> for String {
  type Error = crate::SqlError;
  fn try_from(table_manager: TableManager) -> Result<String, Self::Error> {
    table_manager.to_sql().to_sql_string()
  }
}