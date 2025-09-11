use diesel::prelude::*;
use crate::schema::city;

#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = city)]
pub struct CityDb {
  pub city_id: i32,
  pub name: String,
  pub country: String,
  pub state: Option<String>,
  pub longitude: f64,
  pub latitude: f64,
}

#[derive(Insertable)]
#[diesel(table_name = city)]
pub struct NewCity {
  pub name: String,
  pub country: String,
  pub state: Option<String>,
  pub longitude: f64,
  pub latitude: f64,
}

impl CityDb {
  pub fn full_name(&self) -> String {
    match &self.state {
      Some(state) => format!("{}, {}, {}", self.name, state, self.country),
      None => format!("{}, {}", self.name, self.country),
    }
  }
}