use crate::table::column::{self, column_type::{self, ColumnTypeable}, Column};
use crate::collectors::Sql;

enum ActionTable {
  CreateTable,
  AltertTable { new_name: Option<String> },
  DropTable,
}

pub struct DefineTable {
  action: Option<ActionTable>,
  pub name: String,
  pub charset: Option<String>,
  pub engine: Option<String>,
  pub collation: Option<String>,
  // storage: Option<String>,
  comment: Option<String>,
  // foreign key checks
  pub foreign_key_constraint_attribute: Option<ReferenceOption>,
  pub columns: Vec<Column>,
}

impl Default for DefineTable {
  fn default() -> DefineTable {
    DefineTable {
      action: None,
      name: "".to_owned(),
      // MyISAM | InnoDB
      engine: None,
      charset: None,
      collation: None,
      // DISK | MEMORY
      // storage: None,
      comment: None,
      foreign_key_constraint_attribute: None, // Some("ON UPDATE CASCADE".to_owned()),
      columns: vec![],
    }
  }
}

impl DefineTable {
  // mysql: https://dev.mysql.com/doc/refman/8.0/en/create-table.html
  pub fn create(name: &str, callback: impl Fn(&mut Self) -> ()) -> Self {
    let mut table = DefineTable::default();
    table.name = name.to_owned();
    table.action = Some(ActionTable::CreateTable);
    // 父表更新时子表也更新，父表删除时如果子表有匹配的项，删除失败
    table.foreign_key_constraint_attribute = Some(ReferenceOption::Custom("ON UPDATE CASCADE ON DELETE RESTRICT".to_owned()));
    callback(&mut table);
    table
  }
  // mysql: https://dev.mysql.com/doc/refman/8.0/en/alter-table.html
  fn _alter(name: &str, new_name: Option<&str>, callback: impl Fn(&mut Self) -> ()) -> Self {
    let mut table = DefineTable::default();
    table.name = name.to_owned();
    if let Some(new_name) = new_name {
      table.action = Some(ActionTable::AltertTable { new_name: Some(new_name.to_owned()) });
    } else {
      table.action = Some(ActionTable::AltertTable { new_name: None });
    }
    callback(&mut table);
    table
  }
  pub fn rename(name: &str, new_name: &str) -> Self {
    Self::_alter(name, Some(new_name), |_| {})
  }
  pub fn alter(name: &str, callback: impl Fn(&mut Self) -> ()) -> Self {
    Self::_alter(name, None, callback)
  }
  // mysql: https://dev.mysql.com/doc/refman/8.0/en/drop-database.html
  pub fn drop(name: &str) -> Self {
    let mut table = DefineTable::default();
    table.name = name.to_owned();
    table.action = Some(ActionTable::DropTable);
    table
  }

  pub fn set_column_boolean(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, boolean_callback: impl Fn(&mut column_type::Boolean) -> ()) {
    let mut boolean = column_type::Boolean::default();
    boolean_callback(&mut boolean);
    self.columns.push(column::Column::new(column_name, column_type_action, boolean.into()));
  }
  pub fn add_column_boolean(&mut self, column_name: &str, boolean_callback: impl Fn(&mut column_type::Boolean) -> ()) {
    self.set_column_boolean(column_name, column::ColumnTypeAction::AddColumn { position: None }, boolean_callback)
  }
  pub fn modify_column_boolean(&mut self, column_name: &str, boolean_callback: impl Fn(&mut column_type::Boolean) -> ()) {
    self.set_column_boolean(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, boolean_callback)
  }
  pub fn change_column_boolean(&mut self, column_name: &str, new_column_name: &str, boolean_callback: impl Fn(&mut column_type::Boolean) -> ()) {
    self.set_column_boolean(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, boolean_callback)
  }
  pub fn rename_column_boolean(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_boolean(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_boolean(&mut self, column_name: &str) {
    self.set_column_boolean(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }


  pub fn set_column_integer(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, integer_callback: impl Fn(&mut column_type::Integer) -> ()) {
    let mut integer = column_type::Integer::default();
    integer_callback(&mut integer);
    self.columns.push(column::Column::new(column_name, column_type_action, integer.into()));
  }
  pub fn add_column_integer(&mut self, column_name: &str, integer_callback: impl Fn(&mut column_type::Integer) -> ()) {
    self.set_column_integer(column_name, column::ColumnTypeAction::AddColumn { position: None, }, integer_callback)
  }
  pub fn modify_column_integer(&mut self, column_name: &str, integer_callback: impl Fn(&mut column_type::Integer) -> ()) {
    self.set_column_integer(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, integer_callback)
  }
  pub fn change_column_integer(&mut self, column_name: &str, new_column_name: &str, integer_callback: impl Fn(&mut column_type::Integer) -> ()) {
    self.set_column_integer(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, integer_callback)
  }
  pub fn rename_column_integer(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_integer(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_integer(&mut self, column_name: &str) {
    self.set_column_integer(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }


  pub fn set_column_float(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, float_callback: impl Fn(&mut column_type::Float) -> ()) {
    let mut float = column_type::Float::default();
    float_callback(&mut float);
    self.columns.push(column::Column::new(column_name, column_type_action, float.into()));
  }
  pub fn add_column_float(&mut self, column_name: &str, float_callback: impl Fn(&mut column_type::Float) -> ()) {
    self.set_column_float(column_name, column::ColumnTypeAction::AddColumn { position: None }, float_callback);
  }
  pub fn modify_column_float(&mut self, column_name: &str, float_callback: impl Fn(&mut column_type::Float) -> ()) {
    self.set_column_float(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, float_callback);
  }
  pub fn change_column_float(&mut self, column_name: &str, new_column_name: &str, float_callback: impl Fn(&mut column_type::Float) -> ()) {
    self.set_column_float(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, float_callback);
  }
  pub fn rename_column_float(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_float(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_float(&mut self, column_name: &str) {
    self.set_column_float(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_double(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, double_callback: impl Fn(&mut column_type::Double) -> ()) {
    let mut double = column_type::Double::default();
    double_callback(&mut double);
    self.columns.push(column::Column::new(column_name, column_type_action, double.into()));
  }
  pub fn add_column_double(&mut self, column_name: &str, double_callback: impl Fn(&mut column_type::Double) -> ()) {
    self.set_column_double(column_name, column::ColumnTypeAction::AddColumn { position: None }, double_callback)
  }
  pub fn modify_column_double(&mut self, column_name: &str, double_callback: impl Fn(&mut column_type::Double) -> ()) {
    self.set_column_double(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, double_callback);
  }
  pub fn change_column_double(&mut self, column_name: &str, new_column_name: &str, double_callback: impl Fn(&mut column_type::Double) -> ()) {
    self.set_column_double(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, double_callback);
  }
  pub fn rename_column_double(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_double(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_double(&mut self, column_name: &str) {
    self.set_column_double(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_decimal(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, decimal_callback: impl Fn(&mut column_type::Decimal) -> ()) {
    let mut decimal = column_type::Decimal::default();
    decimal_callback(&mut decimal);
    self.columns.push(column::Column::new(column_name, column_type_action, decimal.into()));
  }
  pub fn add_column_decimal(&mut self, column_name: &str, decimal_callback: impl Fn(&mut column_type::Decimal) -> ()) {
    self.set_column_decimal(column_name, column::ColumnTypeAction::AddColumn { position: None }, decimal_callback)
  }
  pub fn modify_column_decimal(&mut self, column_name: &str, decimal_callback: impl Fn(&mut column_type::Decimal) -> ()) {
    self.set_column_decimal(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, decimal_callback)
  }
  pub fn change_column_decimal(&mut self, column_name: &str, new_column_name: &str, decimal_callback: impl Fn(&mut column_type::Decimal) -> ()) {
    self.set_column_decimal(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, decimal_callback)
  }
  pub fn rename_column_decimal(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_decimal(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_decimal(&mut self, column_name: &str) {
    self.set_column_decimal(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_string(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, string_callback: impl Fn(&mut column_type::ColumnString) -> ()) {
    let mut string = column_type::ColumnString::default();
    string_callback(&mut string);
    self.columns.push(column::Column::new(column_name, column_type_action, string.into()));
  }
  pub fn add_column_string(&mut self, column_name: &str, string_callback: impl Fn(&mut column_type::ColumnString) -> ()) {
    self.set_column_string(column_name, column::ColumnTypeAction::AddColumn { position: None }, string_callback)
  }
  pub fn modify_column_string(&mut self, column_name: &str, string_callback: impl Fn(&mut column_type::ColumnString) -> ()) {
    self.set_column_string(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, string_callback)
  }
  pub fn change_column_string(&mut self, column_name: &str, new_column_name: &str, string_callback: impl Fn(&mut column_type::ColumnString) -> ()) {
    self.set_column_string(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None}, string_callback)
  }
  pub fn rename_column_string(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_string(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_string(&mut self, column_name: &str) {
    self.set_column_string(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_text(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, text_callback: impl Fn(&mut column_type::Text) -> ()) {
    let mut text = column_type::Text::default();
    text_callback(&mut text);
    self.columns.push(column::Column::new(column_name, column_type_action, text.into()));
  }
  pub fn add_column_text(&mut self, column_name: &str, text_callback: impl Fn(&mut column_type::Text) -> ()) {
    self.set_column_text(column_name, column::ColumnTypeAction::AddColumn { position: None }, text_callback)
  }
  pub fn modify_column_text(&mut self, column_name: &str, text_callback: impl Fn(&mut column_type::Text) -> ()) {
    self.set_column_text(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, text_callback)
  }
  pub fn change_column_text(&mut self, column_name: &str, new_column_name: &str, text_callback: impl Fn(&mut column_type::Text) -> ()) {
    self.set_column_text(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, text_callback)
  }
  pub fn rename_column_text(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_text(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_text(&mut self, column_name: &str) {
    self.set_column_text(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_time(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, time_callback: impl Fn(&mut column_type::Time) -> ()) {
    let mut time = column_type::Time::default();
    time_callback(&mut time);
    self.columns.push(column::Column::new(column_name, column_type_action, time.into()));
  }
  pub fn add_column_time(&mut self, column_name: &str, time_callback: impl Fn(&mut column_type::Time) -> ()) {
    self.set_column_time(column_name, column::ColumnTypeAction::AddColumn { position: None }, time_callback)
  }
  pub fn modify_column_time(&mut self, column_name: &str, time_callback: impl Fn(&mut column_type::Time) -> ()) {
    self.set_column_time(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, time_callback)
  }
  pub fn change_column_time(&mut self, column_name: &str, new_column_name: &str, time_callback: impl Fn(&mut column_type::Time) -> ()) {
    self.set_column_time(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, time_callback)
  }
  pub fn rename_column_time(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_time(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_time(&mut self, column_name: &str) {
    self.set_column_time(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_date(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, date_callback: impl Fn(&mut column_type::Date) -> ()) {
    let mut date = column_type::Date::default();
    date_callback(&mut date);
    self.columns.push(column::Column::new(column_name, column_type_action, date.into()));
  }
  pub fn add_column_date(&mut self, column_name: &str, date_callback: impl Fn(&mut column_type::Date) -> ()) {
    self.set_column_date(column_name, column::ColumnTypeAction::AddColumn { position: None }, date_callback)
  }
  pub fn modify_column_date(&mut self, column_name: &str, date_callback: impl Fn(&mut column_type::Date) -> ()) {
    self.set_column_date(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, date_callback)
  }
  pub fn change_column_date(&mut self, column_name: &str, new_column_name: &str, date_callback: impl Fn(&mut column_type::Date) -> ()) {
    self.set_column_date(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, date_callback)
  }
  pub fn rename_column_date(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_date(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_date(&mut self, column_name: &str) {
    self.set_column_date(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_datetime(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, datetime_callback: impl Fn(&mut column_type::Datetime) -> ()) {
    let mut datetime = column_type::Datetime::default();
    datetime_callback(&mut datetime);
    self.columns.push(column::Column::new(column_name, column_type_action, datetime.into()));
  }
  pub fn add_column_datetime(&mut self, column_name: &str, datetime_callback: impl Fn(&mut column_type::Datetime) -> ()) {
    self.set_column_datetime(column_name, column::ColumnTypeAction::AddColumn { position: None }, datetime_callback)
  }
  pub fn modify_column_datetime(&mut self, column_name: &str, datetime_callback: impl Fn(&mut column_type::Datetime) -> ()) {
    self.set_column_datetime(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, datetime_callback)
  }
  pub fn change_column_datetime(&mut self, column_name: &str, new_column_name: &str, datetime_callback: impl Fn(&mut column_type::Datetime) -> ()) {
    self.set_column_datetime(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, datetime_callback)
  }
  pub fn rename_column_datetime(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_datetime(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_datetime(&mut self, column_name: &str) {
    self.set_column_datetime(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_timestamp(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, timestamp_callback: impl Fn(&mut column_type::Timestamp) -> ()) {
    let mut timestamp = column_type::Timestamp::default();
    timestamp_callback(&mut timestamp);
    self.columns.push(column::Column::new(column_name, column_type_action, timestamp.into()));
  }
  pub fn add_column_timestamp(&mut self, column_name: &str, timestamp_callback: impl Fn(&mut column_type::Timestamp) -> ()) {
    self.set_column_timestamp(column_name, column::ColumnTypeAction::AddColumn { position: None }, timestamp_callback)
  }
  pub fn modify_column_timestamp(&mut self, column_name: &str, timestamp_callback: impl Fn(&mut column_type::Timestamp) -> ()) {
    self.set_column_timestamp(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, timestamp_callback)
  }
  pub fn change_column_timestamp(&mut self, column_name: &str, new_column_name: &str, timestamp_callback: impl Fn(&mut column_type::Timestamp) -> ()) {
    self.set_column_timestamp(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, timestamp_callback)
  }
  pub fn rename_column_timestamp(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_timestamp(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_timestamp(&mut self, column_name: &str) {
    self.set_column_timestamp(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_json(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, json_callback: impl Fn(&mut column_type::Json) -> ()) {
    let mut json = column_type::Json::default();
    json_callback(&mut json);
    self.columns.push(column::Column::new(column_name, column_type_action, json.into()));
  }
  pub fn add_column_json(&mut self, column_name: &str, json_callback: impl Fn(&mut column_type::Json) -> ()) {
    self.set_column_json(column_name, column::ColumnTypeAction::AddColumn { position: None }, json_callback)
  }
  pub fn modify_column_json(&mut self, column_name: &str, json_callback: impl Fn(&mut column_type::Json) -> ()) {
    self.set_column_json(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, json_callback)
  }
  pub fn change_column_json(&mut self, column_name: &str, new_column_name: &str, json_callback: impl Fn(&mut column_type::Json) -> ()) {
    self.set_column_json(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, json_callback)
  }
  pub fn rename_column_json(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_json(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_json(&mut self, column_name: &str) {
    self.set_column_json(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_blob(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, blob_callback: impl Fn(&mut column_type::Blob) -> ()) {
    let mut blob = column_type::Blob::default();
    blob_callback(&mut blob);
    self.columns.push(column::Column::new(column_name, column_type_action, blob.into()));
  }
  pub fn add_column_blob(&mut self, column_name: &str, blob_callback: impl Fn(&mut column_type::Blob) -> ()) {
    self.set_column_blob(column_name, column::ColumnTypeAction::AddColumn { position: None }, blob_callback)
  }
  pub fn modify_column_blob(&mut self, column_name: &str, blob_callback: impl Fn(&mut column_type::Blob) -> ()) {
    self.set_column_blob(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, blob_callback)
  }
  pub fn change_column_blob(&mut self, column_name: &str, new_column_name: &str, blob_callback: impl Fn(&mut column_type::Blob) -> ()) {
    self.set_column_blob(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None }, blob_callback)
  }
  pub fn rename_column_blob(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_blob(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_blob(&mut self, column_name: &str) {
    self.set_column_blob(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_column_binary(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, binary_callback: impl Fn(&mut column_type::Binary) -> ()) {
    let mut binary = column_type::Binary::default();
    binary_callback(&mut binary);
    self.columns.push(column::Column::new(column_name, column_type_action, binary.into()));
  }
  pub fn add_column_binary(&mut self, column_name: &str, binary_callback: impl Fn(&mut column_type::Binary) -> ()) {
    self.set_column_binary(column_name, column::ColumnTypeAction::AddColumn { position: None }, binary_callback)
  }
  pub fn modify_column_binary(&mut self, column_name: &str, binary_callback: impl Fn(&mut column_type::Binary) -> ()) {
    self.set_column_binary(column_name, column::ColumnTypeAction::ModifyColumn { position: None }, binary_callback)
  }
  pub fn change_column_binary(&mut self, column_name: &str, new_column_name: &str, binary_callback: impl Fn(&mut column_type::Binary) -> ()) {
    self.set_column_binary(column_name, column::ColumnTypeAction::ChangeColumn { new_name: new_column_name.into(), position: None}, binary_callback)
  }
  pub fn rename_column_binary(&mut self, column_name: &str, new_column_name: &str) {
    self.set_column_binary(column_name, column::ColumnTypeAction::RenameColumn { new_name: new_column_name.into(), position: None}, |_| {})
  }
  pub fn drop_column_binary(&mut self, column_name: &str) {
    self.set_column_binary(column_name, column::ColumnTypeAction::DropColumn, |_| {})
  }

  pub fn set_index(&mut self, column_names: Vec<&str>, column_type_action: column::ColumnTypeAction, index_callback: impl Fn(&mut column_type::Index) -> ()) {
    let mut index = column_type::Index::default();
    index_callback(&mut index);

    let mut column = column::Column::new("", column_type_action, index.into());
    column.column_names = column_names.into_iter().map(|name| name.into()).collect::<Vec<String>>();
    self.columns.push(column);
  }
  pub fn add_index(&mut self, column_names: Vec<&str>, index_callback: impl Fn(&mut column_type::Index) -> ()) {
    assert!(column_names.len() >= 1);
    self.set_index(column_names, column::ColumnTypeAction::AddConstraint, index_callback);
  }
  pub fn rename_index(&mut self, index_name: &str, new_index_name: &str) {
    self.set_index(vec![index_name], column::ColumnTypeAction::RenameIndex { new_name: new_index_name.into() }, |_| {})
  }
  pub fn drop_index(&mut self, index_name: &str) {
    self.set_index(vec![index_name], column::ColumnTypeAction::DropConstraint, |_| {});
  }

  pub fn set_unique(&mut self, column_names: Vec<&str>, column_type_action: column::ColumnTypeAction, unique_callback: impl Fn(&mut column_type::Unique) -> ()) {
    let mut unique = column_type::Unique::default();
    unique_callback(&mut unique);

    let mut column = column::Column::new("", column_type_action, unique.into());
    column.column_names = column_names.into_iter().map(|name| name.into()).collect::<Vec<String>>();
    self.columns.push(column);
  }
  pub fn add_unique(&mut self, column_names: Vec<&str>, unique_callback: impl Fn(&mut column_type::Unique) -> ()) {
    self.set_unique(column_names, column::ColumnTypeAction::AddConstraint, unique_callback);
  }
  pub fn drop_unique(&mut self, unique_name: &str) {
    self.set_unique(vec![unique_name], column::ColumnTypeAction::DropConstraint, |_| {});
  }

  pub fn set_primary_key(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, primary_key_callback: impl Fn(&mut column_type::PrimaryKey) -> ()) {
    let mut primary_key = column_type::PrimaryKey::default();
    primary_key.set_foreign_key(column_name.into());
    primary_key_callback(&mut primary_key);
    self.columns.push(column::Column::new(column_name, column_type_action, primary_key.into()));
  }
  pub fn add_primary_key(&mut self, column_name: &str, primary_key_callback: impl Fn(&mut column_type::PrimaryKey) -> ()) {
    self.set_primary_key(column_name, column::ColumnTypeAction::AddConstraint, primary_key_callback);
  }
  pub fn drop_primary_key(&mut self, column_name: &str) {
    self.set_primary_key(column_name, column::ColumnTypeAction::DropConstraint, |_| {});
  }

  pub fn set_foreign_key(&mut self, column_name: &str, column_type_action: column::ColumnTypeAction, foreign_key_callback: impl Fn(&mut column_type::ForeignKey) -> ()) {
    let mut foreign_key = column_type::ForeignKey::default();
    foreign_key.set_foreign_key(column_name.into());
    foreign_key_callback(&mut foreign_key);
    self.columns.push(column::Column::new(column_name, column_type_action, foreign_key.into()));
  }
  pub fn add_foreign_key(&mut self, column_name: &str, foreign_key_callback: impl Fn(&mut column_type::ForeignKey) -> ()) {
    self.set_foreign_key(column_name, column::ColumnTypeAction::AddConstraint, foreign_key_callback);
  }
  pub fn drop_foreign_key(&mut self, foreign_key_name: &str) {
    self.set_foreign_key(foreign_key_name, column::ColumnTypeAction::DropConstraint, |_| {});
  }

  pub fn to_sql(&self) -> Sql {
    match &self.action {
      Some(ActionTable::CreateTable) => {
        let mut sql = Sql::new(format!("CREATE TABLE IF NOT EXISTS {}", self.name));
        sql.push(' ').push('(').push('\n');
        // to_sql
        {
          let mut column_append_idx = -1;
          for column in self.columns.iter() {
            if let Some(column_sql) = column.to_sql(&self) {
              if column_append_idx >= 0 {
                sql.push(',').push('\n');
               }
              sql.push_sql(&column_sql);
              column_append_idx += 1;
            }
          }
        }
        // to_constraint_sql
        {
          for column in self.columns.iter() {
            let ret = column.to_constraint_sql(self);
            if let Some(constraint_sql) = ret {
              sql.push(',').push('\n').push_sql(&constraint_sql);
            }
          }
        }
        sql.push('\n').push(')');
        if let Some(engine) = &self.engine {
          sql.push_value(&format!(" ENGINE SET {}", engine));
        }
        if let Some(charset) = &self.charset {
          sql.push_value(&format!(" CHARACTER SET {}", charset));
        }
        if let Some(collation) = &self.collation {
          sql.push_value(&format!(" COLLATE {}", collation));
        }
        if let Some(comment) = &self.comment {
          sql.push_value(&format!(" COLLATE {}", comment));
        }
        sql.push(';');
        sql
      },
      Some(ActionTable::AltertTable { new_name}) => {
        let mut sql = Sql::new(format!("ALTER TABLE {}", self.name));
        let mut column_append_idx = -1;
        // to_sql
        {
          // sql.push_value(&format!("id {} NOT NULL {} PRIMARY KEY", crate::const_data::GLOBAL_KEY_MAPPING.get("integer").unwrap(), crate::const_data::GLOBAL_KEY_MAPPING.get("auto_increment").unwrap()));
          for column in self.columns.iter() {
            match &column.column_type_action {
              column_type::ColumnTypeAction::AddColumn { .. } => {
                if let Some(column_sql) = column.to_sql(self) {
                  if column_append_idx >= 0 { sql.push_value(",\n"); } else { sql.push('\n'); }
                  sql.push_value("ADD COLUMN ").push_sql(&column_sql);
                  column_append_idx += 1;
                }
              },
              column_type::ColumnTypeAction::ModifyColumn { .. } => {
                if let Some(column_sql) = column.to_sql(self) {
                  if column_append_idx >= 0 { sql.push_value(",\n"); } else { sql.push('\n'); }
                  sql.push_value("MODIFY COLUMN ").push_sql(&column_sql);
                  column_append_idx += 1;
                }
              },
              column_type::ColumnTypeAction::ChangeColumn { .. } => {
                if let Some(column_sql) = column.to_sql(self) {
                  if column_append_idx >= 0 { sql.push_value(",\n"); } else { sql.push('\n'); }
                  sql.push_value("CHANGE COLUMN ").push_sql(&column_sql);
                  column_append_idx += 1;
                }
              },
              column_type::ColumnTypeAction::RenameColumn { .. } => {
                if let Some(column_sql) = column.to_sql(self) {
                  if column_append_idx >= 0 { sql.push_value(",\n"); } else { sql.push('\n'); }
                  sql.push_value("RENAME COLUMN ").push_sql(&column_sql);
                  column_append_idx += 1;
                }
              },
              column_type::ColumnTypeAction::DropColumn => {
                if let Some(column_sql) = column.to_sql(self) {
                  if column_append_idx >= 0 { sql.push_value(",\n"); } else { sql.push('\n'); }
                  sql.push_value("DROP COLUMN ").push_sql(&column_sql);
                  column_append_idx += 1;
                }
              },
              column_type::ColumnTypeAction::AddConstraint |
              column_type::ColumnTypeAction::DropConstraint |
              column_type::ColumnTypeAction::RenameIndex { .. } => ()
            }
          }
        }
        // to_constraint_sql
        {
          for column in self.columns.iter() {
            let ret = column.to_constraint_sql(self);
            if let Some(constraint_sql) = ret {
              if column_append_idx >= 0 {
                sql.push(',');
              }
              sql.push('\n');
              match column.column_type_action {
                column_type::ColumnTypeAction::AddConstraint => {
                  sql.push_value("ADD CONSTRAINT ").push_sql(&constraint_sql);
                },
                _ => {
                  sql.push_sql(&constraint_sql);
                }
              }
            }
          }
        }
        if let Some(new_name) = new_name {
          sql.push_value(&format!(" RENAME TO {}", new_name));
        }
        if let Some(engine) = &self.engine {
          sql.push_value(&format!(" ENGINE = {}", engine));
        }
        if let Some(charset) = &self.charset {
          sql.push_value(&format!(" CHARACTER SET {}", charset));
        }
        if let Some(collation) = &self.collation {
          sql.push_value(&format!(" COLLATE {}", collation));
        }
        sql.push(';');
        sql
      },
      Some(ActionTable::DropTable) => {
        Sql::new(format!("DROP TABLE IF EXISTS {};", self.name))
      },
      None => {
        Sql::default()
      }
    }
  }
}

impl TryFrom<DefineTable> for String {
  type Error = crate::SqlError;
  fn try_from(table: DefineTable) -> Result<String, Self::Error> {
    table.to_sql().to_sql_string()
  }
}

#[derive(Clone, Debug)]
pub enum ReferenceOption {
  NoAction,
  SetNull,
  Cascade,
  Restrict,
  Custom(String),
}
impl ReferenceOption {
  fn to_sql(&self) -> Sql {
    match self {
      ReferenceOption::NoAction => Sql::new("NO ACTION".to_owned()),
      ReferenceOption::SetNull => Sql::new("SET NULL".to_owned()),
      // 父表更新时子表也更新，父表删除时子表匹配的项也删除
      ReferenceOption::Cascade => Sql::new("CANSCADE".to_owned()),
      ReferenceOption::Restrict => Sql::new("RESTRICT".to_owned()),
      ReferenceOption::Custom(custom) => Sql::new(custom.to_owned()),
    }
  }
}
// https://dev.mysql.com/doc/refman/5.6/en/create-table-foreign-keys.html
// user_id => ForeignKey { column_names: vec!["user_id".into()] index_name: Some("fk_on_user_id".into()), reference_table_name: "users".into(), reference_table_column_names: vec!["id".into()], reference_option: None }
#[derive(Clone, Debug)]
pub struct ForeignKey {
  column_names: Vec<String>,
  index_name: Option<String>,
  reference_table_name: String, // eg: user_id => users
  reference_table_column_names: Vec<String>, // id
  reference_on_update_option: Option<ReferenceOption>,
  reference_on_delete_option: Option<ReferenceOption>,
}

impl From<&str> for ForeignKey {
  fn from(column_name: &str) -> Self {
    ForeignKey {
      column_names: vec![column_name.into()],
      index_name: Some(format!("fk_on_{}", column_name)),
      reference_table_name: inflector::string::pluralize::to_plural(regex::Regex::new(r"_id$").unwrap().replace(&column_name, "").as_ref()),
      reference_table_column_names: vec!["id".into()],
      reference_on_update_option: None,
      reference_on_delete_option: None,
    }
  }
}

impl ForeignKey {
  pub fn to_sql(&self, column: &crate::Column) -> Sql {
    let mut sql = Sql::new("FOREIGN KEY".to_owned());
    if let Some(index_name) = &self.index_name {
      sql.push_value(&format!(" {}", index_name));
    }
    if column.column_type_action == column_type::ColumnTypeAction::DropConstraint {
      return sql;
    }
    sql.push_value(&format!(" ({}) REFERENCES {} ({})", self.column_names.join(","), self.reference_table_name, self.reference_table_column_names.join(",")));
    if let Some(reference_on_update_option) = &self.reference_on_update_option {
      sql.push(' ').push_sql(&reference_on_update_option.to_sql());
    }
    if let Some(reference_on_delete_option) = &self.reference_on_delete_option {
      sql.push(' ').push_sql(&reference_on_delete_option.to_sql());
    }
    sql
  }
}