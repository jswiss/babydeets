use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::babies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Baby {
  pub id: String,
  pub name: String,
  pub sex: String,
  pub birthday: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}

#[derive(serde::Deserialize)]
pub struct NewBaby {
  pub name: String,
  pub sex: String,
  pub birthday: String
}
