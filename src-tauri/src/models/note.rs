use crate::schema::notes;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
  pub id: i32,
  pub baby_id: i32,
  pub note: Option<String>,
  pub image: Option<Vec<u8>>,
  pub file: Option<Vec<u8>>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}
