use std::sync::Arc;

use axum::{Router, routing::{get, post, delete}};

use crate::services::DbPool;

mod cities;
mod users;

pub fn create_routes() -> Router<Arc<DbPool>> {
  Router::new()
    // Root route
    .route("/", get(|| async { "Planet Birthday API - v1.0" }))
      
    // City routes
    .route("/api/cities", get(cities::get_cities))
    .route("/api/city", post(cities::create_city))
    .route("/api/city", delete(cities::delete_city))
      
    // User routes
    .route("/api/users", get(users::get_users))
    .route("/api/users/register", post(users::register_user))
    .route("/api/users/{id}", get(users::get_user_by_id))
    .route("/api/users/{id}", delete(users::delete_user))
    .route("/api/users/city/{city_id}", get(users::get_users_by_city))
      
    // Birthday routes
    .route("/api/birthdays/today", get(users::get_birthday_users_today))
}