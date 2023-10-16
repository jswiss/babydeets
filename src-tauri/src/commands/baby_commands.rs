use uuid::Uuid;

use crate::{models::baby::{NewBaby, Baby}, services::baby_service, fns::string_date_now::string_date_now};

#[tauri::command]
pub fn create_baby(new_baby: NewBaby) {
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

#[tauri::command]
pub fn upload_baby_files(files: Vec<String>) -> Result<(), String> {
  let image_dir = std::path::PathBuf::from("images");
  std::fs::create_dir_all(&image_dir).map_err(|e| format!("Failed to create image directory: {:?}", e))?;

  for file in files {
    let src = std::path::PathBuf::from(&file);
    let dest = image_dir.join(src.file_name().ok_or("Invalid file path")?);
    std::fs::copy(&src, &dest).map_err(|e| format!("Failed to copy file: {:?}", e))?;
}

  Ok(())
}
