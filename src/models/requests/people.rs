use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize, Clone, Debug)]
pub struct RegisterUserRequest {
  pub name: String,
  pub student_id: String,
  pub city_id: i32,
  pub date_of_birth: NaiveDate,
}

impl RegisterUserRequest {
  pub fn validate(&self) -> Result<(), String> {
    if self.name.trim().is_empty() {
      return Err("Name cannot be empty".to_string());
    }

    if self.student_id.trim().is_empty() {
      return Err("Student ID cannot be empty".to_string());
    }

    let today = chrono::Utc::now().naive_utc().date();
    if self.date_of_birth > today {
      return Err("Birth date cannot be in the future".to_string());
    }

    if self.date_of_birth < NaiveDate::from_ymd_opt(1912, 1, 1).unwrap() {
      return Err("Birth date too far in the past".to_string());
    }

    Ok(())
  }
}