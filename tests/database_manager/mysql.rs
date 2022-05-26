use sql_gen::SqlGen;

fn test_create_database() -> Result<(), sql_gen::SqlError> {
  let sql: String = SqlGen::create_database("db_prod").try_into()?;
  assert_eq!(sql, "CREATE DATABASE db_prod CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci;");

  Ok(())
}

fn test_drop_database() -> Result<(), sql_gen::SqlError> {
  let sql: String = SqlGen::drop_database("db_prod").try_into()?;
  assert_eq!(sql, "DROP DATABASE db_prod;");

  Ok(())
}

async fn main_test() -> Result<(), sql_gen::SqlError> {
  test_create_database()?;
  test_drop_database()?;
  Ok(())
}
#[test]
fn test_database_manager() {
    assert!(tokio_test::block_on(main_test()).is_ok());
}