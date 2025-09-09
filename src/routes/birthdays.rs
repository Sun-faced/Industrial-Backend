use axum::Json;

use crate::models::responses::birthday::GetBirthdayResponse;

pub async fn get_todays_birthdays() -> Json<Vec<GetBirthdayResponse>> {
  let all_people = vec! [
    GetBirthdayResponse {
      name: "Name1".to_string(),
      full_city_name: "City1".to_string(),
      longitude: 40.0, 
      latitude: 60.0,
    },
    GetBirthdayResponse {
      name: "Name2".to_string(),
      full_city_name: "City2".to_string(),
      longitude: 41.0, 
      latitude: 61.0,
    },
    GetBirthdayResponse {
      name: "Name3".to_string(),
      full_city_name: "City3".to_string(),
      longitude: 44.0, 
      latitude: 65.0,
    },
  ];

  Json(all_people)

}