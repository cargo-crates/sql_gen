use crate::collectors::Sql;

enum ActionDatabase {
  CreateDatabase,
  AlterDatabase,
  DropDatabase,
}


pub struct DefineDatabase {
  action: Option<ActionDatabase>,
  database_name: String,
  pub charset: Option<String>,
  pub collation: Option<String>,
  // 'Y' | 'N'
  pub encryption: Option<char>,
  // 0 | 1
  pub read_only: Option<u8>,
}

impl Default for DefineDatabase {
  fn default() -> DefineDatabase {
    DefineDatabase {
      action: None,
      database_name: "".to_owned(),
      charset: None, // "utf8mb4".to_owned(),
      collation: None, // "utf8mb4_0900_ai_ci".to_owned(),
      encryption: None,
      read_only: None,
    }
  }
}

impl DefineDatabase {
  // mysql: https://dev.mysql.com/doc/refman/8.0/en/create-database.html
  pub fn create_database(database_name: &str) -> Self {
    let mut database = DefineDatabase::default();
    database.action = Some(ActionDatabase::CreateDatabase);
    database.database_name = database_name.to_owned();
    database.charset = Some("utf8mb4".to_owned());
    database.collation = Some("utf8mb4_0900_ai_ci".to_owned());
    database
  }
  // mysql: https://dev.mysql.com/doc/refman/8.0/en/alter-database.html
  pub fn alter_database(database_name: &str, callback: impl Fn(&mut Self) -> ()) -> Self {
    let mut database = DefineDatabase::default();
    database.action = Some(ActionDatabase::AlterDatabase);
    database.database_name = database_name.to_owned();
    callback(&mut database);
    database
  }
  // mysql: https://dev.mysql.com/doc/refman/8.0/en/drop-database.html
  pub fn drop_database(database_name: &str) -> Self {
    let mut database = DefineDatabase::default();
    database.action = Some(ActionDatabase::DropDatabase);
    database.database_name = database_name.to_owned();
    database
  }

  pub fn to_sql(&self) -> Sql {
    match &self.action {
      Some(ActionDatabase::CreateDatabase) => {
        let mut sql = Sql::new(format!("CREATE DATABASE IF NOT EXISTS {}", self.database_name));
        if let Some(charset) = &self.charset {
          sql.push_value(&format!(" CHARACTER SET {}", charset));
        }
        if let Some(collation) = &self.collation {
          sql.push_value(&format!(" COLLATE {}", collation));
        }
        if let Some(encryption) = &self.encryption {
          sql.push_value(&format!(" ENCRYPTION {}", encryption));
        }
        sql.push(';');
        sql
      },
      Some(ActionDatabase::AlterDatabase) => {
        let mut sql = Sql::new(format!("ALTER DATABASE {}", self.database_name));
        if let Some(charset) = &self.charset {
          sql.push_value(&format!(" CHARACTER SET {}", charset));
        }
        if let Some(collation) = &self.collation {
          sql.push_value(&format!(" COLLATE {}", collation));
        }
        if let Some(encryption) = &self.encryption {
          sql.push_value(&format!(" ENCRYPTION '{}'", encryption));
        }
        if let Some(read_only) = &self.read_only {
          sql.push_value(&format!(" READ ONLY {}", read_only));
        }
        sql.push(';');
        sql
      },
      Some(ActionDatabase::DropDatabase) => {
        Sql::new(format!("DROP DATABASE IF EXISTS {};", self.database_name))
      },
      None => Sql::default()
    }
  }
}

impl TryFrom<DefineDatabase> for String {
  type Error = crate::SqlError;
  fn try_from(database_manager: DefineDatabase) -> Result<String, Self::Error> {
    database_manager.to_sql().to_sql_string()
  }
}

#[cfg(test)]
mod tests {
  // use crate::prelude::*;
  use super::*;
  #[test]
  fn to_sql() {
    let db = DefineDatabase::create_database("sql_gen_prod");
    #[cfg(feature = "mysql")]
    {
      assert_eq!(db.to_sql().to_sql_string(), Ok("CREATE DATABASE IF NOT EXISTS sql_gen_prod CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci;".to_owned()));
    }
  }
}