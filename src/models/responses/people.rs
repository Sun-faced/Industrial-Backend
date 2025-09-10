use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct DeletePersonResponse {
  pub success: bool,
  pub result: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUserResponse {
  pub success: bool,
  pub message: String,
  pub user_id: Option<Uuid>,
}