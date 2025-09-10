use serde::{self, Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GetCities {
  pub id: Uuid,
  pub full_city_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCityResponse {
  pub success: bool,
  pub message: String,
  pub city: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteCityResponse {
  pub success: bool,
  pub result: String,
}
