use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize)]
pub struct DeletePersonResponse {
  pub success: bool,
  pub result: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUserResponse {
  pub success: bool,
  pub message: String,
  pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct GetUsers {
  pub id: i32,
  pub name: String,
  pub birthday: NaiveDate,
  pub city_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserResponse {
  pub success: bool,
  pub user: Option<GetUsers>,
  pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct BirthdayUsersResponse {
  pub success: bool,
  pub users: Vec<GetUsers>,
  pub count: usize,
  pub message: String,
}