use diesel::prelude::*;
use std::sync::Arc;
use chrono::{NaiveDate, Datelike};

use crate::services::DbPool;
use crate::schema::users::dsl::*;
use crate::models::entities::people::{UserDb, NewUser};
use crate::models::responses::people::{GetUsers, RegisterUserResponse};
use crate::models::requests::people::RegisterUserRequest;

#[derive(Debug)]
pub enum UserServiceError {
  DatabaseError,
  NotFound,
  ValidationError(String),
  InvalidStudentId,
}


pub async fn get_all_users(pool: Arc<DbPool>) -> Result<Vec<GetUsers>, UserServiceError> {
  let mut conn = pool.get().map_err(|_| UserServiceError::DatabaseError)?;
  
  let users_result = users
    .select(UserDb::as_select())
    .load::<UserDb>(&mut conn);
      
  match users_result {
    Ok(user_list) => {
      let response: Vec<GetUsers> = user_list
        .into_iter()
        .map(|user_db| GetUsers {
          id: user_db.id,
          name: user_db.name,
          birthday: user_db.birthday,
          city_id: user_db.city_id,
        })
        .collect();
      Ok(response)
    }
    Err(_) => Err(UserServiceError::DatabaseError),
  }
}

pub async fn register_new_user(
    pool: Arc<DbPool>, 
    user_request: RegisterUserRequest
) -> Result<RegisterUserResponse, UserServiceError> {
  
  let mut conn = pool.get().map_err(|_| UserServiceError::DatabaseError)?;
  
  let new_user = NewUser {
    id: user_request.student_id, 
    name: user_request.name.clone(),
    birthday: user_request.date_of_birth,
    city_id: user_request.city_id,
  };

  let insert_result = diesel::insert_into(users)
    .values(&new_user)
    .get_result::<UserDb>(&mut conn);

  match insert_result {
    Ok(created_user) => Ok(RegisterUserResponse {
      success: true,
      message: format!("User registered successfully with ID: {}", created_user.id),
      user_id: Some(created_user.id),
    }),
    Err(_) => Err(UserServiceError::DatabaseError),
  }
}

pub async fn delete_user_by_id(
    pool: Arc<DbPool>, 
    user_id: i32
) -> Result<bool, UserServiceError> {
  
  let mut conn = pool.get().map_err(|_| UserServiceError::DatabaseError)?;
  
  let delete_result = diesel::delete(users.filter(id.eq(user_id)))
    .execute(&mut conn);

  match delete_result {
    Ok(rows_deleted) => Ok(rows_deleted > 0),
    Err(_) => Err(UserServiceError::DatabaseError),
  }
}

pub async fn get_user_by_id(pool: Arc<DbPool>, user_id: i32) -> Result<UserDb, UserServiceError> {
  
  let mut conn = pool.get().map_err(|_| UserServiceError::DatabaseError)?;
  
  users.filter(id.eq(user_id))
      .select(UserDb::as_select())
      .first::<UserDb>(&mut conn)
      .map_err(|_| UserServiceError::NotFound)
}

pub async fn get_users_by_city(pool: Arc<DbPool>, city_id_param: i32) -> Result<Vec<UserDb>, UserServiceError> {
  
  let mut conn = pool.get().map_err(|_| UserServiceError::DatabaseError)?;
  
  users.filter(city_id.eq(city_id_param))
      .select(UserDb::as_select())
      .load::<UserDb>(&mut conn)
      .map_err(|_| UserServiceError::DatabaseError)
}

pub async fn get_users_with_birthday_today(pool: Arc<DbPool>) -> Result<Vec<UserDb>, UserServiceError> {
  
  let mut conn = pool.get().map_err(|_| UserServiceError::DatabaseError)?;
  let today = chrono::Utc::now().naive_utc().date();
  
  // Note: I know that this is a simplified and dumb approach.
  let all_users_result = users
    .select(UserDb::as_select())
    .load::<UserDb>(&mut conn);
    
  match all_users_result {
    Ok(all_users) => {
      let birthday_users: Vec<UserDb> = all_users
        .into_iter()
        .filter(|user| {
          user.birthday.month() == today.month() && user.birthday.day() == today.day()
        })
        .collect();
      Ok(birthday_users)
    }
    Err(_) => Err(UserServiceError::DatabaseError),
  }
}