use sql_gen::SqlGen;

fn test_rename_database() -> Result<(), sql_gen::SqlError> {
  let sql: String = SqlGen::rename_database("db_prod", "new_db_prod").try_into()?;
  assert_eq!(sql, "ALTER DATABASE db_prod SET SINGLE_USER WITH ROLLBACK IMMEDIATE;\nALTER DATABASE db_prod MODIFY NAME = new_db_prod;\nALTER DATABASE new_db_prod SET MULTI_USER;");

  Ok(())
}

async fn main_test() -> Result<(), sql_gen::SqlError> {
  test_rename_database()?;
  Ok(())
}
#[test]
fn test_database_manager() {
    assert!(tokio_test::block_on(main_test()).is_ok());
}