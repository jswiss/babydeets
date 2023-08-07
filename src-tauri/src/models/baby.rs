use crate::schema::babies;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::babies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Baby {
  pub id: i32,
  pub name: String,
  pub sex: String,
  pub birthday: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}
