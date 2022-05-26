use crate::collectors::Sql;

enum Action {
  Create,
  #[cfg(any(feature = "postgresql", feature = "mssql"))]
  Rename(String),
  Drop,
}


pub struct DatabaseManager {
  action: Action,
  database_name: String,
  pub charset: String,
  pub collation: String,
}

impl Default for DatabaseManager {
  fn default() -> DatabaseManager {
    DatabaseManager {
      action: Action::Create,
      database_name: "".to_owned(),
      charset: "utf8mb4".to_owned(),
      collation: "utf8mb4_0900_ai_ci".to_owned(),
    }
  }
}

impl DatabaseManager {
  pub fn create_database(database_name: &str) -> Self {
    let mut database_manager = DatabaseManager::default();
    database_manager.action = Action::Create;
    database_manager.database_name = database_name.to_owned();
    database_manager
  }
  #[cfg(any(feature = "postgresql", feature = "mssql"))]
  pub fn rename_database(old_database_name: &str, database_name: &str) -> Self {
    let mut database_manager = DatabaseManager::default();
    database_manager.action = Action::Rename(old_database_name.to_owned());
    database_manager.database_name = database_name.to_owned();
    database_manager
  }
  pub fn drop_database(database_name: &str) -> Self {
    let mut database_manager = DatabaseManager::default();
    database_manager.action = Action::Drop;
    database_manager.database_name = database_name.to_owned();
    database_manager
  }

  pub fn to_sql(&self) -> Sql {
    match &self.action {
      Action::Create => {
        let mut sql = Sql::new(format!("CREATE DATABASE {}", self.database_name));
        #[cfg(not(feature = "sqlite"))]
        sql.push_value(&format!(" CHARACTER SET {} COLLATE {}", self.charset, self.collation));
        sql.push(';');
        sql
      },
      #[cfg(any(feature = "postgresql", feature = "mssql"))]
      Action::Rename(old_database_name) => {
        #[cfg(feature = "postgresql")]
        {
          Sql::new(format!("ALTER DATABASE {} RENAME TO {};", old_database_name, self.database_name))
        }
        #[cfg(feature = "mssql")]
        {
          let mut sql = Sql::new(format!("ALTER DATABASE {} SET SINGLE_USER WITH ROLLBACK IMMEDIATE;\n", old_database_name));
          sql.push_value(&format!("ALTER DATABASE {} MODIFY NAME = {};\n", old_database_name, self.database_name));
          sql.push_value(&format!("ALTER DATABASE {} SET MULTI_USER;", self.database_name));
          sql
        }
      },
      Action::Drop => {
        Sql::new(format!("DROP DATABASE {};", self.database_name))
      },
    }
  }
}

impl TryFrom<DatabaseManager> for String {
  type Error = crate::SqlError;
  fn try_from(database_manager: DatabaseManager) -> Result<String, Self::Error> {
    database_manager.to_sql().to_sql_string()
  }
}