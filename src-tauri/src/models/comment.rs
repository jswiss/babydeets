use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Comment {
  pub id: i32,
  pub note_id: i32,
  pub comment: Option<String>,
  pub image: Option<Vec<u8>>,
  pub file: Option<Vec<u8>>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}
