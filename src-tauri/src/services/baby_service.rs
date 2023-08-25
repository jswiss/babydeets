use crate::{db::establish_db_connection, models::baby::Baby, schema::babies, schema::babies::dsl};
use crate::fns::string_date_now;

use diesel::prelude::*;

pub fn add_baby(new_baby: &Baby) {
  let conn = &mut establish_db_connection();
  diesel::insert_into(babies::table)
    .values(new_baby)
    .execute(conn)
    .expect("Error saving new baby");
}

pub fn list_babies() -> Vec<Baby> {
  let conn = &mut establish_db_connection();

  dsl::babies
  .order_by(dsl::created_at.desc())
  .load::<Baby>(conn)
  .expect("Error loading babies")
}

pub fn get_baby(id: String) -> Option<Baby> {
  let conn = &mut establish_db_connection();

  let result = dsl::babies.filter(dsl::id.eq(id))
    .first::<Baby>(conn)
    .ok();

  result
}

pub fn update_baby(updated_baby: &Baby) {
  let conn = &mut establish_db_connection();

  diesel::update(dsl::babies.filter(dsl::id.eq(&updated_baby.id)))
    .set((dsl::name.eq(&updated_baby.name), dsl::photo.eq(&updated_baby.photo), dsl::sex.eq(&updated_baby.sex), dsl::birthday.eq(&updated_baby.birthday)))
    .execute(conn)
    .expect("Error updating baby");
}
