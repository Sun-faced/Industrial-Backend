use axum::http::StatusCode;
use axum::Json;
use crate::models::responses::cities::{GetCities, CreateCityResponse};
use crate::models::requests::cities::CreateCityRequest;
use uuid::Uuid;

pub async fn get_cities() -> Json<Vec<GetCities>> {
  let all_cities = vec![
    GetCities {
      id: Uuid::new_v4(),
      full_city_name: "City1".to_string(),
    },
    GetCities {
      id: Uuid::new_v4(),
      full_city_name: "City2".to_string(),
    },
    GetCities {
      id: Uuid::new_v4(),
      full_city_name: "City3".to_string(),
    },
  ];

  Json(all_cities)
}

pub async fn create_city(Json(City): Json<CreateCityRequest>) -> Result<(StatusCode, Json<CreateCityResponse>), StatusCode> {
  match City.validate() {
    Ok(true) => {
      return Ok((
          StatusCode::CREATED,
          Json(CreateCityResponse {
              success: true,
              message: "City created successfully".to_string(),
              city: Some(City.name),
          })
      ))
    }

    Ok(false) => {
      Err(StatusCode::BAD_GATEWAY)
    }

    Err(error) => {
      return Ok ((
          StatusCode::BAD_REQUEST,
          Json(CreateCityResponse {
              success: false,
              message: error.to_string(),
              city: None,
          })
      ))
    }
  }
}