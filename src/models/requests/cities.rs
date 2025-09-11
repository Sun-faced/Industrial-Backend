use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct CreateCityRequest {
  pub name: String,
  pub state_province: Option<String>,
  pub country: String,
  pub latitude: f64,
  pub longitude: f64,
}

impl CreateCityRequest {
  fn is_lat_valid(&self) -> bool {
    self.latitude <= 90.0 && self.latitude >= -90.0
  }

  fn is_lon_valid(&self) -> bool {
    self.longitude <= 180.0 && self.longitude >= -180.0
  }

  pub fn validate(&self) -> Result<(), String> {
    if self.name.trim().is_empty() {
      return Err("Name can't be empty".to_string());
    }

    if self.country.trim().is_empty() {
      return Err("Country can't be empty".to_string());
    }

    if !self.is_lat_valid() || !self.is_lon_valid() {
      return Err("Wrong coordinates".to_string());
    }

    Ok(())
  }
}

#[derive(Deserialize)]
pub struct DeleteCityRequst {
  pub id: i32,
  pub name: String,
}

impl DeleteCityRequst {
    pub fn validate(&self) -> Result<(), String> {
      println!("{} ", self.id);
      println!("{} ", self.name);
      Ok(())
    }
}