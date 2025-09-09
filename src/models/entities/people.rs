use serde::{self, Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, Datelike};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Person {
  pub id: Uuid,
  pub name: String,
  pub birth_date: NaiveDate,
  pub city_id: Uuid,
}

impl Person {
  pub fn new(id: Uuid, name: String, birth_date: NaiveDate, city_id: Uuid) -> Self {
    Self {
      id,
      name,
      birth_date,
      city_id
    }
  }

  pub fn is_birthday_today(&self) -> bool {
    let today = chrono::Utc::now().naive_utc().date();
    self.birth_date.month() == today.month() && self.birth_date.day() == today.day()
  }
}

