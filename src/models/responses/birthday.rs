use serde::{self, Serialize};

#[derive(Serialize, Debug)]
pub struct GetBirthdayResponse {
  pub name: String,
  pub full_city_name: String,
  pub longitude: f64,
  pub latitude: f64,
}
