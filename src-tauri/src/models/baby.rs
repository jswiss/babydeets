use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Serialize, serde::Deserialize, Selectable, Insertable)]
#[diesel(table_name = crate::schema::babies)]
#[derive(Debug)]
pub struct Baby {
  pub id: String,
  pub name: String,
  pub photo: Option<Vec<u8>>,
  pub sex: String,
  pub birthday: String,
  pub created_at: String,
  pub updated_at: String
}
