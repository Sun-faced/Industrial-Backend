use axum::{Json, http::StatusCode, extract::{Path, State}};
use std::sync::Arc;

use crate::services::DbPool;
use crate::services::people as user_service;
use crate::models::requests::people::RegisterUserRequest;
use crate::models::responses::people::{
  RegisterUserResponse, DeletePersonResponse, GetUsers, 
  GetUserResponse, BirthdayUsersResponse
};

pub async fn get_users(
    State(pool): State<Arc<DbPool>>
) -> Result<Json<Vec<GetUsers>>, StatusCode> {
  
  match user_service::get_all_users(pool).await {
    Ok(users) => Ok(Json(users)),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}

pub async fn register_user(
    State(pool): State<Arc<DbPool>>,
    Json(request): Json<RegisterUserRequest>
) -> Result<(StatusCode, Json<RegisterUserResponse>), StatusCode> {
    
    if let Err(error) = request.validate(pool.clone()).await {
      return Ok((
        StatusCode::BAD_REQUEST,
        Json(RegisterUserResponse {
          success: false,
          message: error,
          user_id: None,
        })
      ));
    }

  match user_service::register_new_user(pool, request).await {
    Ok(response) => Ok((StatusCode::CREATED, Json(response))),
    Err(user_service::UserServiceError::InvalidStudentId) => Ok((
      StatusCode::FORBIDDEN,
      Json(RegisterUserResponse {
        success: false,
        message: "Invalid student ID".to_string(),
        user_id: None,
      })
    )),
    Err(user_service::UserServiceError::DatabaseError) => Ok((
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(RegisterUserResponse {
        success: false,
        message: "Failed to register user".to_string(),
        user_id: None,
      })
    )),
    Err(user_service::UserServiceError::ValidationError(msg)) => Ok((
      StatusCode::BAD_REQUEST,
      Json(RegisterUserResponse {
        success: false,
        message: msg,
        user_id: None,
      })
    )),
    Err(_) => Ok((
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(RegisterUserResponse {
        success: false,
        message: "Unexpected error during user registration".to_string(),
        user_id: None,
      })
    )),
  }
}

pub async fn delete_user(
    State(pool): State<Arc<DbPool>>,
    Path(user_id): Path<i32>
) -> Result<(StatusCode, Json<DeletePersonResponse>), StatusCode> {
  
  match user_service::delete_user_by_id(pool, user_id).await {
    Ok(true) => Ok((
      StatusCode::OK,
      Json(DeletePersonResponse {
        success: true,
        result: "User deleted successfully".to_string(),
      }),
    )),
    Ok(false) => Ok((
      StatusCode::NOT_FOUND,
      Json(DeletePersonResponse {
        success: false,
        result: "User not found".to_string(),
      }),
    )),
    Err(_) => Ok((
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(DeletePersonResponse {
        success: false,
        result: "Failed to delete user".to_string(),
      }),
    ))
  }
}

pub async fn get_user_by_id(
    State(pool): State<Arc<DbPool>>,
    Path(user_id): Path<i32>
) -> Result<(StatusCode, Json<GetUserResponse>), StatusCode> {
  
  match user_service::get_user_by_id(pool, user_id).await {
    Ok(user) => Ok((
      StatusCode::OK,
      Json(GetUserResponse {
        success: true,
        user: Some(GetUsers {
          id: user.id,
          name: user.name,
          birthday: user.birthday,
          city_id: user.city_id,
        }),
        message: "User found".to_string(),
      }),
    )),
    Err(user_service::UserServiceError::NotFound) => Ok((
      StatusCode::NOT_FOUND,
      Json(GetUserResponse {
        success: false,
        user: None,
        message: "User not found".to_string(),
      }),
    )),
    Err(_) => Ok((
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(GetUserResponse {
        success: false,
        user: None,
        message: "Failed to retrieve user".to_string(),
      }),
    ))
  }
}

pub async fn get_users_by_city(
    State(pool): State<Arc<DbPool>>,
    Path(city_id): Path<i32>
) -> Result<Json<Vec<GetUsers>>, StatusCode> {
  
  match user_service::get_users_by_city(pool, city_id).await {
    Ok(users) => {
      let response: Vec<GetUsers> = users
        .into_iter()
        .map(|user| GetUsers {
          id: user.id,
          name: user.name,
          birthday: user.birthday,
          city_id: user.city_id,
        })
        .collect();
      Ok(Json(response))
    }
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}

pub async fn get_birthday_users_today(
    State(pool): State<Arc<DbPool>>
) -> Result<(StatusCode, Json<BirthdayUsersResponse>), StatusCode> {
  
  match user_service::get_users_with_birthday_today(pool).await {
    Ok(users) => {
      let user_list: Vec<GetUsers> = users
        .into_iter()
        .map(|user| GetUsers {
          id: user.id,
          name: user.name,
          birthday: user.birthday,
          city_id: user.city_id,
        })
        .collect();
      
      let count = user_list.len();
      let message = if count == 0 {
        "No users have birthdays today".to_string()
      } else if count == 1 {
        "1 user has a birthday today".to_string()
      } else {
        format!("{} users have birthdays today", count)
      };
      
      Ok((
        StatusCode::OK,
        Json(BirthdayUsersResponse {
          success: true,
          users: user_list,
          count,
          message,
        }),
      ))
    }
    Err(_) => Ok((
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(BirthdayUsersResponse {
        success: false,
        users: vec![],
        count: 0,
        message: "Failed to retrieve birthday users".to_string(),
      }),
    ))
  }
}