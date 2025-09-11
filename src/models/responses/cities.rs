use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GetCities {
  pub id: i32,
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
