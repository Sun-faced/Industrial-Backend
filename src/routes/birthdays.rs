use axum::Json;
use crate::models::responses::birthday::GetBirthdayResponse;
use crate::routes::users::get_all_users;
use crate::routes::cities::get_city_by_id;

pub async fn get_todays_birthdays() -> Json<Vec<GetBirthdayResponse>> {
  let users = get_all_users();
  let mut birthday_responses = Vec::new();

  for user in users {
    if user.is_birthday_today() {
      if let Some(city) = get_city_by_id(user.city_id).await {
        birthday_responses.push(GetBirthdayResponse {
          name: user.name,
          full_city_name: city.full_name(),
          longitude: city.longitude,
          latitude: city.latitude,
        });
      }
    }
  }

  Json(birthday_responses)
}