use sql_gen::SqlGen;

fn test_create_table() -> Result<(), sql_gen::SqlError> {
  let sql: String = SqlGen::create_table("users").try_into()?;
  assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (\nid INTEGER NOT NULL AUTOINCREMENT PRIMARY KEY\n);");

  Ok(())
}

fn test_rename_table() -> Result<(), sql_gen::SqlError> {
  let sql: String = SqlGen::rename_table("users", "new_users").try_into()?;
  assert_eq!(sql, "ALTER TABLE users RENAME TO new_users;");

  Ok(())
}

async fn main_test() -> Result<(), sql_gen::SqlError> {
  test_create_table()?;
  test_rename_table()?;
  Ok(())
}
#[test]
fn test_table_manager() {
    assert!(tokio_test::block_on(main_test()).is_ok());
}