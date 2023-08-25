use uuid::Uuid;

use crate::{models::baby::Baby, services::baby_service, fns::string_date_now::string_date_now};

#[tauri::command]
pub fn create_baby(new_baby: Baby) {
  println!("Baby {} was born on {}", new_baby.name, new_baby.birthday);

  let now = string_date_now();
  let updated_now = now.clone();

  let baby = Baby {
    id: Uuid::new_v4().to_string(),
    name: new_baby.name,
    sex: new_baby.sex,
    birthday: new_baby.birthday,
    photo: new_baby.photo,
    created_at: now,
    updated_at: updated_now,
  };

  baby_service::add_baby(&baby);
}

#[tauri::command]
pub fn list_babies() -> Vec<Baby> {
  baby_service::list_babies()
}

#[tauri::command]
pub fn get_baby(id: String) -> Option<Baby> {
  baby_service::get_baby(id)
}

#[tauri::command]
pub fn update_baby(updated_baby: Baby) {
  baby_service::update_baby(&updated_baby);
}
