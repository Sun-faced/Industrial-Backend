use serde::{self, Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct City {
  pub id: Uuid,
  pub name: String,
  pub country: String,
  pub state_province: Option<String>,
  pub latitude: f64,
  pub longitude: f64,
}

impl City {
  pub fn new(
    name: String,
    country: String, 
    state_province: Option<String>,
    latitude: f64, 
    longitude: f64,
  ) -> Self {
    Self {
      id: Uuid::new_v4(),
      name,
      country,
      state_province,
      latitude,
      longitude,
    }
  }

  pub fn full_name(&self) -> String {
    match &self.state_province {
      Some(state) => format!("{}, {}, {}", self.name, state, self.country),
      None => format!("{}, {}", self.name, self.country),
    }
  }
}