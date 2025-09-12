use diesel::prelude::*;
use std::sync::Arc;

use crate::models::requests::DeleteCityRequst;
use crate::services::DbPool;
use crate::schema::city::dsl::*;
use crate::models::entities::cities::{CityDb, NewCity};
use crate::models::responses::cities::{GetCities, CreateCityResponse};
use crate::models::requests::cities::CreateCityRequest;

#[derive(Debug)]
pub enum CityServiceError {
  DatabaseError,
  NotFound,
}

pub async fn get_all_cities(pool: Arc<DbPool>) -> Result<Vec<GetCities>, CityServiceError> {
  
  let mut conn = pool.get().map_err(|_| CityServiceError::DatabaseError)?;
  
  let cities_result = city
    .select(CityDb::as_select())
    .load::<CityDb>(&mut conn);
      
  match cities_result {
    Ok(cities) => {
      let response: Vec<GetCities> = cities
        .into_iter()
        .map(|city_db| GetCities {
          id: city_db.city_id,
          full_city_name: city_db.full_name(),
        })
        .collect();
      Ok(response)
    }
    Err(_) => Err(CityServiceError::DatabaseError),
  }
}

pub async fn create_new_city(
    pool: Arc<DbPool>, 
    city_request: CreateCityRequest
) -> Result<CreateCityResponse, CityServiceError> {    
  let mut conn = pool.get().map_err(|_| CityServiceError::DatabaseError)?;
  
  let new_city = NewCity {
    name: city_request.name.clone(),
    country: city_request.country.clone(),
    state: city_request.state_province.clone(),
    longitude: city_request.longitude,
    latitude: city_request.latitude,
  };

  let insert_result = diesel::insert_into(city)
    .values(&new_city)
    .get_result::<CityDb>(&mut conn);

  match insert_result {
    Ok(created_city) => Ok(CreateCityResponse {
      success: true,
      message: format!("City created successfully with ID: {}", created_city.city_id),
      city: Some(city_request.name),
    }),
    Err(_) => Err(CityServiceError::DatabaseError),
  }
}

pub async fn delete_city_by_id(
    pool: Arc<DbPool>, 
    given_city_id: i32
) -> Result<bool, CityServiceError> {
  
  let mut conn = pool.get().map_err(|_| CityServiceError::DatabaseError)?;
  
  let delete_result = diesel::delete(city.filter(city_id.eq(given_city_id)))
    .execute(&mut conn);

  match delete_result {
    Ok(rows_deleted) => Ok(rows_deleted > 0),
    Err(_) => Err(CityServiceError::DatabaseError),
  }
}

pub async fn check_delete_request(
    pool: Arc<DbPool>,
    city_request: &DeleteCityRequst
) -> Result<bool, CityServiceError> {
  let mut conn = pool.get().map_err(|_| CityServiceError::DatabaseError)?;
  
  // Check if a city exists with both the given ID and name
  let city_exists = city
    .filter(city_id.eq(city_request.id))
    .filter(name.eq(&city_request.name))
    .select(city_id)
    .first::<i32>(&mut conn);
  
  match city_exists {
      Ok(_) => Ok(true),
      Err(diesel::NotFound) => Ok(false),
      Err(_) => Err(CityServiceError::DatabaseError),
    }
}

pub async fn get_city_by_id(pool: Arc<DbPool>, city_id_param: i32) -> Result<CityDb, CityServiceError> {
  
  let mut conn = pool.get().map_err(|_| CityServiceError::DatabaseError)?;
  
  city.filter(city_id.eq(city_id_param))
      .select(CityDb::as_select())
      .first::<CityDb>(&mut conn)
      .map_err(|_| CityServiceError::NotFound)
}

pub async fn get_city_by_name(pool: Arc<DbPool>, city_name: &str) -> Result<CityDb, CityServiceError> {
  
  let mut conn = pool.get().map_err(|_| CityServiceError::DatabaseError)?;
  
  city.filter(name.eq(city_name))
    .select(CityDb::as_select())
    .first::<CityDb>(&mut conn)
    .map_err(|_| CityServiceError::NotFound)
}