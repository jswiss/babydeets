use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
  pub id: String,
  pub baby_id: String,
  pub note: Option<String>,
  pub image: Option<String>,
  pub file: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}
