use serde::{self, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct GetCities {
  pub id: Uuid,
  pub full_city_name: String,
}

#[derive(Serialize)]
pub struct CreateCityResponse {
  pub success: bool,
  pub message: String,
  pub city: Option<String>,
}