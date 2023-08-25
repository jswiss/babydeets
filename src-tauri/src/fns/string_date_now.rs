pub fn string_date_now() -> String {
  let now = chrono::Utc::now().naive_utc();

  let str_now = now.format("%Y-%m-%d %H:%M:%S.%f").to_string();
  str_now
}
