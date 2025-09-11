use axum::Json;
use crate::models::responses::birthday::GetBirthdayResponse;
use crate::routes::users::get_all_users;
use crate::services::cities::get_city_by_id;

pub async fn get_todays_birthdays() -> bool {
  return false;
}