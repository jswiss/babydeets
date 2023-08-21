use uuid::Uuid;

use crate::{models::baby::{Baby, NewBaby}, services::baby_service};

#[tauri::command]
pub fn create_baby(new_baby: NewBaby) {
  println!("Baby {} was born on {}", new_baby.name, new_baby.birthday);

  let baby = Baby {
    id: Uuid::new_v4().to_string(),
    name: new_baby.name,
    sex: new_baby.sex,
    birthday: new_baby.birthday,
    created_at: string_date(),
  };

  baby_service::add_baby(&baby);
}

#[tauri::command]
pub fn list_babies() -> Vec<Baby> {
  let babies = baby_service::list_babies();

  println!("Baby {} is in the table, he is a {}", babies[0].name, babies[0].sex);
  babies
}

fn string_date() -> String {
  let now = chrono::Utc::now().naive_utc();

  let str_now = now.format("%Y-%m-%d %H:%M:%S.%f").to_string();
  str_now
}
