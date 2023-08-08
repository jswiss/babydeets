use uuid::Uuid;

use crate::{models::baby::{Baby, NewBaby}, services::baby_service::add_baby};

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn create_baby(new_baby: NewBaby) {
  let baby = Baby {
    id: Uuid::new_v4().to_string(),
    name: new_baby.name,
    sex: new_baby.sex,
    birthday: new_baby.birthday,
    created_at: chrono::Utc::now().naive_utc(),
    updated_at: chrono::Utc::now().naive_utc()
  };

  add_baby(&baby);
}
