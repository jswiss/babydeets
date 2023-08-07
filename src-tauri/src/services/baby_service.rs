use crate::{db::establish_db_connection, models::babies::{Baby, NewBaby}, schema::babies, schema::babies::dsl};

use diesel::prelude::*;

pub fn create_baby(new_baby: &NewBaby) {
  let conn = establish_db_connection();

  diesel::insert_into(babies::table)
    .values(new_baby)
    .execute(conn)
    .expect("Error saving new baby");
}
