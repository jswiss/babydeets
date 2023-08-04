use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
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

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
  pub id: i32,
  pub baby_id: i32,
  pub note: Option<String>,
  pub image: Option<Vec<u8>>,
  pub file: Option<Vec<u8>>
}
