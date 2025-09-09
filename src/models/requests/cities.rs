use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug)]
pub struct CreateCityRequest {
  pub name: String,
  state_province: Option<String>,
  country: String,
  latitude: f64,
  longitude: f64,
}

impl CreateCityRequest {
  fn is_lat_valid(&self) -> bool {
    return self.latitude <= 90.0 && self.latitude >= -90.0
  }

  fn is_lon_valid(&self) -> bool {
    return self.longitude <= 180.0 && self.longitude >= -180.0
  }

  pub fn validate(&self) -> Result<bool, String> {
    if self.name.trim().is_empty() {
      return Err("Name can't be empty".to_string())
    }

    if self.country.trim().is_empty() {
      return Err("Country can't be empty".to_string())
    }

    if (!self.is_lat_valid()) || (!self.is_lon_valid()) {
      return Err("Wrong coordinates".to_string())
    }

    Ok(true)
  }
}