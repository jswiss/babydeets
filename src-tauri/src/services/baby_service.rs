use crate::{db::establish_db_connection, models::baby::Baby, schema::babies};

use diesel::prelude::*;

pub fn add_baby(new_baby: &Baby) {
  let conn = &mut establish_db_connection();

  diesel::insert_into(babies::table)
    .values(new_baby)
    .execute(conn)
    .expect("Error saving new baby");
}
