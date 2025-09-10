use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::models::requests::DeleteCityRequst;
use crate::models::responses::cities::{GetCities, CreateCityResponse};
use crate::models::requests::cities::CreateCityRequest;
use crate::models::responses::DeleteCityResponse;
use crate::models::entities::City;

// In-memory storage for cities
lazy_static::lazy_static! {
  static ref CITIES: Arc<Mutex<HashMap<Uuid, City>>> = {
    let mut cities = HashMap::new();
    
    let city1 = City::new(
      "New York".to_string(),
      "United States".to_string(),
      Some("NY".to_string()),
      40.7128,
      -74.0060
    );
    let city2 = City::new(
      "London".to_string(),
      "United Kingdom".to_string(),
      None,
      51.5074,
      -0.1278
    );
    let city3 = City::new(
      "Tokyo".to_string(),
      "Japan".to_string(),
      None,
      35.6762,
      139.6503
    );
  
  cities.insert(city1.id, city1);
  cities.insert(city2.id, city2);
  cities.insert(city3.id, city3);
  
  Arc::new(Mutex::new(cities))
  };
}

pub async fn get_cities() -> Json<Vec<GetCities>> {
  let cities = CITIES.lock().unwrap();
  let all_cities: Vec<GetCities> = cities
    .values()
    .map(|city| GetCities {
      id: city.id,
      full_city_name: city.full_name(),
    })
    .collect();

  Json(all_cities)
}

pub async fn create_city(Json(city_request): Json<CreateCityRequest>) -> Result<(StatusCode, Json<CreateCityResponse>), StatusCode> {
  match city_request.validate() {
    Ok(()) => {
      let city = City::new(
        city_request.name.clone(),
        city_request.country.clone(),
        city_request.state_province.clone(),
        city_request.latitude,
        city_request.longitude,
      );

      let mut cities = CITIES.lock().unwrap();
      cities.insert(city.id, city);

      Ok((
        StatusCode::CREATED,
        Json(CreateCityResponse {
          success: true,
          message: "City created successfully".to_string(),
          city: Some(city_request.name),
        })
      ))
    }
    Err(error) => {
      Ok((
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

pub async fn delete_city(Json(city): Json<DeleteCityRequst>) -> Result<(StatusCode, Json<DeleteCityResponse>), StatusCode> {
  match city.validate() {
    Ok(()) => {
      Ok((
        StatusCode::ACCEPTED,
        Json(DeleteCityResponse { 
          success: true, 
          result: "Deleted successfully".to_string() 
        }),
      ))
    }
    Err(error) => {
      Ok((
        StatusCode::NOT_FOUND,
        Json(DeleteCityResponse { 
          success: false, 
          result: error 
        }),
      ))
    }
  }
}

// Helper function to get city by ID
pub async fn get_city_by_id(city_id: Uuid) -> Option<City> {
  let cities = CITIES.lock().unwrap();
  cities.get(&city_id).cloned()
}