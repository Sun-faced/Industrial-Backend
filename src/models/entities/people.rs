use diesel::prelude::*;
use chrono::{NaiveDate, Datelike};
use serde::{Serialize};
use crate::schema::users;

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = users)]
pub struct UserDb {
  pub id: i32,
  pub name: String,
  pub birthday: NaiveDate,
  pub city_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
  pub id: i32,
  pub name: String,
  pub birthday: NaiveDate,
  pub city_id: i32,
}

impl UserDb {
  pub fn is_birthday_today(&self) -> bool {
    let today = chrono::Utc::now().naive_utc().date();
    self.birthday.month() == today.month() && self.birthday.day() == today.day()
  }
  
  pub fn age(&self) -> i32 {
    let today = chrono::Utc::now().naive_utc().date();
    let mut age = today.year() - self.birthday.year();
    
    if today.month() < self.birthday.month() || 
       (today.month() == self.birthday.month() && today.day() < self.birthday.day()) {
      age -= 1;
    }
    
    age
  }
}
