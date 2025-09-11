use axum::http::StatusCode;
use axum::Json;
use axum::extract::State;
use std::sync::Arc;

use crate::services::DbPool;
use crate::services::cities as city_service;
use crate::models::requests::DeleteCityRequst;
use crate::models::responses::cities::{GetCities, CreateCityResponse};
use crate::models::requests::cities::CreateCityRequest;
use crate::models::responses::DeleteCityResponse;

pub async fn get_cities(State(pool): State<Arc<DbPool>>) -> Result<Json<Vec<GetCities>>, StatusCode> {
  println!("🔍 Route: get_cities called");
  
  match city_service::get_all_cities(pool).await {
    Ok(cities) => {
      println!("✅ Route: Service returned {} cities", cities.len());
      Ok(Json(cities))
    },
    Err(e) => {
      println!("❌ Route: Service failed with error: {:?}", e);
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    },
  }
}

pub async fn create_city(
    State(pool): State<Arc<DbPool>>,
    Json(city_request): Json<CreateCityRequest>
) -> Result<(StatusCode, Json<CreateCityResponse>), StatusCode> {
    // Validate the request
    if let Err(error) = city_request.validate() {
      return Ok((
        StatusCode::BAD_REQUEST,
        Json(CreateCityResponse {
          success: false,
          message: error,
          city: None,
        })
      ));
    }

  match city_service::create_new_city(pool, city_request).await {
    Ok(response) => Ok((StatusCode::CREATED, Json(response))),
    Err(city_service::CityServiceError::DatabaseError) => Ok((
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(CreateCityResponse {
        success: false,
        message: "Failed to create city".to_string(),
        city: None,
      })
    )),
    Err(city_service::CityServiceError::NotFound) => Ok((
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(CreateCityResponse {
        success: false,
        message: "Unexpected error during city creation".to_string(),
        city: None,
      })
    )),
  }
}

pub async fn delete_city(
    State(pool): State<Arc<DbPool>>,
    Json(delete_request): Json<DeleteCityRequst>
) -> Result<(StatusCode, Json<DeleteCityResponse>), StatusCode> {
  if let Err(error) = delete_request.validate() {
    return Ok((
      StatusCode::BAD_REQUEST,
      Json(DeleteCityResponse {
        success: false,
        result: error,
      }),
    ));
  }

  match city_service::delete_city_by_id(pool, delete_request.id).await {
    Ok(true) => Ok((
      StatusCode::OK,
      Json(DeleteCityResponse {
        success: true,
        result: "City deleted successfully".to_string(),
      }),
    )),
    Ok(false) => Ok((
      StatusCode::NOT_FOUND,
      Json(DeleteCityResponse {
        success: false,
        result: "City not found".to_string(),
      }),
    )),
    Err(_) => Ok((
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(DeleteCityResponse {
        success: false,
        result: "Failed to delete city".to_string(),
      }),
    ))
  }
}