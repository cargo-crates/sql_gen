use sql_gen::SqlGen;

fn test_create_database() -> Result<(), sql_gen::SqlError> {
  let sql: String = SqlGen::create_database("db_prod").try_into()?;
  assert_eq!(sql, "CREATE DATABASE db_prod;");

  Ok(())
}

async fn main_test() -> Result<(), sql_gen::SqlError> {
  test_create_database()?;
  Ok(())
}
#[test]
fn test_database_manager() {
    assert!(tokio_test::block_on(main_test()).is_ok());
}