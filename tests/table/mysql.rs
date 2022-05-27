use sql_gen::prelude::*;

fn test_create_table() -> Result<(), sql_gen::SqlError> {
  // id column
  let sql: String = SqlGen::create_table("users", |table| {
    table.add_column_integer("id", |integer| {
      integer.set_null(false).set_auto_increment(true).set_primary_key(true).set_comment("ID");
    })
  }).try_into()?;
  assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (\nid INT NOT NULL AUTO_INCREMENT COMMENT 'ID',\nPRIMARY KEY pk_on_id (id)\n);");

  // integer column
  let sql: String = SqlGen::create_table("users", |table| {
    table.add_column_integer("id", |integer| {
      integer.set_null(false).set_auto_increment(true).set_primary_key(true).set_comment("ID");
    });
    table.add_column_integer("user_id", |integer| {
      integer.set_null(false).set_foreign_key("user_id".into()).set_comment("user_id");
    });
    // boolean
    table.add_column_boolean("is_deleted", |boolean| {
      boolean.set_null(false).set_default(true).set_comment("软删除").set_index(true);
    });
    // integer
    table.add_column_integer("age", |integer| {
      integer.set_unsigned(true).set_null(false).set_default(18).set_comment("年龄");
    });
    // float
    table.add_column_float("price", |float| {
      float.set_null(false).set_default(1.0).set_comment("价格");
    });
    // double
    table.add_column_double("rate", |double| {
      double.set_null(false).set_default(1.0).set_comment("利率");
    });
    // decimal
    table.add_column_decimal("longitude", |decimal| {
      decimal.set_precision_scale(10, 6).set_null(false).set_default(1.0).set_comment("经度");
    });
    table.add_column_decimal("latitude", |decimal| {
      decimal.set_precision_scale(10, 6).set_null(false).set_default(1.0).set_comment("纬度");
    });
    // string
    table.add_column_string("name", |string| {
      string.set_length(200);
    });
    table.add_column_string("email", |string| {
      string.set_null(false).set_index(true).set_unique(true);
    });
    table.add_column_text("remark", |_text| {});
    // time
    table.add_column_time("time_at", |_time| {});
    // date
    table.add_column_date("date_at", |_date| {});
    // // datetime
    table.add_column_datetime("datetime_at", |_datetime| {});
    // // timestamp
    table.add_column_timestamp("created_at", |_time| {});
    // // index
    table.add_index(vec!["name"], |_index| {
      // index.set_unique(true);
    });
    table.add_unique(vec!["username"], |_index| {
      // index.set_unique(true);
    });
    table.add_foreign_key("order_id", |_foreign_key| {});
  }).try_into()?;
  assert_eq!(sql, "CREATE TABLE IF NOT EXISTS users (
id INT NOT NULL AUTO_INCREMENT COMMENT 'ID',
user_id INT NOT NULL COMMENT 'user_id',
is_deleted BOOLEAN NOT NULL DEFAULT true COMMENT '软删除',
age INT UNSIGNED NOT NULL DEFAULT 18 COMMENT '年龄',
price FLOAT NOT NULL DEFAULT 1 COMMENT '价格',
rate DOUBLE NOT NULL DEFAULT 1 COMMENT '利率',
longitude DECIMAL(10, 6) NOT NULL DEFAULT 1 COMMENT '经度',
latitude DECIMAL(10, 6) NOT NULL DEFAULT 1 COMMENT '纬度',
name VARCHAR(200),
email VARCHAR(255) NOT NULL,
remark TEXT,
time_at TIME,
date_at DATE,
datetime_at DATETIME,
created_at TIMESTAMP,
PRIMARY KEY pk_on_id (id),
FOREIGN KEY fk_on_user_id (user_id) REFERENCES users (id),
INDEX index_on_is_deleted (is_deleted),
UNIQUE INDEX unique_index_on_email (email),
INDEX index_on_name (name),
UNIQUE unique_on_username (username),
FOREIGN KEY fk_on_order_id (order_id) REFERENCES orders (id)
);");

  Ok(())
}

fn test_alter_table_columns() ->  Result<(), sql_gen::SqlError> {
  // rename_table
  let sql: String = SqlGen::rename_table("users", "new_users").try_into()?;
  assert_eq!(sql, "ALTER TABLE users RENAME TO new_users;");

  let table = SqlGen::alter_table("users", |table| {
    table.add_column_string("name", |string| {
      string.set_null(false);
    });
    table.modify_column_boolean("is_deleted", |boolean| {
      boolean.set_default(false);
    });
    table.modify_column_integer("age", |integer| {
      integer.set_unsigned(true);
    });
    table.modify_column_float("price", |float| {
      float.set_default(0.0);
    });
    table.modify_column_double("rate", |double| {
      double.set_default(0.0);
    });
    table.modify_column_decimal("rate", |decimal| {
      decimal.set_precision_scale(10, 6);
    });
    table.modify_column_string("phone", |string| {
      string.set_length(20).set_unique(true);
    });
    table.modify_column_text("remark", |text| {
      text.set_default("");
    });
    table.change_column_text("desc", "description", |text| {
      text.set_not_null(true);
    });
    table.rename_column_string("addr", "address");
    table.drop_column_string("email");
    table.add_index(vec!["name", "phone"], |index| { 
      index.set_unique(true);
    });
    table.drop_index("index_on_username");
    table.drop_unique("unique_on_username");
    table.drop_primary_key("id");
    table.drop_foreign_key("order_id");
    table.rename_index("index_on_uid", "idx_on_uid");
  });
  let sql: String = table.try_into()?;
  assert_eq!(sql, "ALTER TABLE users
ADD COLUMN name VARCHAR(255) NOT NULL,
MODIFY COLUMN is_deleted BOOLEAN DEFAULT false,
MODIFY COLUMN age INT UNSIGNED,
MODIFY COLUMN price FLOAT DEFAULT 0,
MODIFY COLUMN rate DOUBLE DEFAULT 0,
MODIFY COLUMN rate DECIMAL(10, 6),
MODIFY COLUMN phone VARCHAR(20),
MODIFY COLUMN remark TEXT DEFAULT ,
CHANGE COLUMN desc description TEXT NOT NULL,
RENAME COLUMN addr TO address,
DROP COLUMN email,
ADD CONSTRAINT UNIQUE INDEX unique_index_on_name_and_phone (name,phone),
DROP INDEX index_on_username,
DROP INDEX unique_on_username,
PRIMARY KEY pk_on_idDROP INDEX id,
FOREIGN KEY fk_on_order_id,
RENAME INDEX index_on_uid TO idx_on_uid;");

  Ok(())
}

fn test_drop_table() -> Result<(), sql_gen::SqlError> {
  let sql: String = SqlGen::drop_table("users").try_into()?;
  assert_eq!(sql, "DROP TABLE IF EXISTS users;");

  Ok(())
}

async fn main_test() -> Result<(), sql_gen::SqlError> {
  test_create_table()?;
  test_alter_table_columns()?;
  test_drop_table()?;
  Ok(())
}
#[test]
fn test_table() {
    assert!(tokio_test::block_on(main_test()).is_ok());
}