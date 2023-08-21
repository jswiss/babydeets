use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Serialize, Selectable, Insertable)]
#[diesel(table_name = crate::schema::babies)]
#[derive(Debug)]
pub struct Baby {
  pub id: String,
  pub name: String,
  pub sex: String,
  pub birthday: String,
  pub created_at: String
}

#[derive(serde::Deserialize)]
pub struct NewBaby {
  pub name: String,
  pub sex: String,
  pub birthday: String
}
