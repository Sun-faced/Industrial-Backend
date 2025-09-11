use axum::{Json, http::StatusCode, extract::Path};
use crate::models::requests::people::RegisterUserRequest;
use crate::models::responses::people::{RegisterUserResponse, DeletePersonResponse};

// In-memory storage for demo purposes
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::entities::Person;

lazy_static::lazy_static! {
  static ref USERS: Arc<Mutex<HashMap<i32, Person>>> = Arc::new(Mutex::new(HashMap::new()));
  static ref STUDENT_IDS: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![
    "STU001".to_string(),
    "STU002".to_string(), 
    "STU003".to_string(),
    "STU004".to_string(),
    "STU005".to_string(),
  ]));
}

pub async fn register_user(Json(request): Json<RegisterUserRequest>) -> Result<(StatusCode, Json<RegisterUserResponse>), StatusCode> {
  match request.validate() {
    Ok(()) => {
      let student_ids = STUDENT_IDS.lock().unwrap();
      if !student_ids.contains(&request.student_id) {
        return Ok((
          StatusCode::FORBIDDEN,
          Json(RegisterUserResponse {
            success: false,
            message: "Invalid student ID".to_string(),
            user_id: None,
          })
        ));
      }
      drop(student_ids);

      let person = Person::new(
        1,
        request.name.clone(),
        request.date_of_birth,
        request.city_id
      );

      let user_id = person.id;
      
      let mut users = USERS.lock().unwrap();
      users.insert(user_id, person);
      drop(users);

      Ok((
        StatusCode::CREATED,
        Json(RegisterUserResponse {
          success: true,
          message: "User registered successfully".to_string(),
          user_id: Some(user_id),
        })
      ))
    }
    Err(error) => {
      Ok((
        StatusCode::BAD_REQUEST,
        Json(RegisterUserResponse {
          success: false,
          message: error,
          user_id: None,
        })
      ))
    }
  }
}

pub async fn delete_user(Path(user_id): Path<i32>) -> Result<(StatusCode, Json<DeletePersonResponse>), StatusCode> {
  let mut users = USERS.lock().unwrap();
  
  match users.remove(&user_id) {
    Some(_) => {
      Ok((
        StatusCode::OK,
        Json(DeletePersonResponse {
          success: true,
          result: "User deleted successfully".to_string(),
        })
      ))
    }
    None => {
      Ok((
        StatusCode::NOT_FOUND,
        Json(DeletePersonResponse {
          success: false,
          result: "User not found".to_string(),
        })
      ))
    }
  }
}

pub fn get_all_users() -> Vec<Person> {
  let users = USERS.lock().unwrap();
  users.values().cloned().collect()
}