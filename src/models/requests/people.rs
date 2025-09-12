use std::sync::Arc;
use serde::Deserialize;
use crate::services::DbPool;
use chrono::NaiveDate;
use crate::services::{people::get_user_by_id, cities::get_city_by_id};

#[derive(Deserialize, Clone, Debug)]
pub struct RegisterUserRequest {
  pub student_id: i32,
  pub name: String,
  pub city_id: i32,
  pub date_of_birth: NaiveDate,
}

impl RegisterUserRequest {
  pub async fn validate(&self, pool: Arc<DbPool>) -> Result<(), String> {
    if self.name.trim().is_empty() {
      return Err("Name cannot be empty".to_string());
    }
    
    let today = chrono::Utc::now().naive_utc().date();
    if self.date_of_birth > today {
    return Err("Birth date cannot be in the future".to_string());
    }
    
    if self.date_of_birth < NaiveDate::from_ymd_opt(1912, 1, 1).unwrap() {
      return Err("Birth date too far in the past".to_string());
    }
    
    match get_user_by_id(pool.clone(), self.student_id).await {
      Ok(_) => return Err("Invalid ID".to_string()),
      Err(_) => {}
    }

    match get_city_by_id(pool, self.city_id).await {
      Ok(_) => Ok(()),
      Err(_) => Err("Invalid city ID".to_string()),
    }
  }
}