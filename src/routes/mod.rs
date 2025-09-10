use axum::{Router, routing::{get, post, delete}};

mod cities;
mod users;
mod birthdays;

pub fn create_routes() -> Router {
  Router::new()
    // Root route
    .route("/", get(|| async { "Planet Birthday API - v1.0" }))
      
    // City routes
    .route("/api/cities", get(cities::get_cities))
    .route("/api/city", post(cities::create_city))
    .route("/api/city", delete(cities::delete_city))
      
    // User routes
    .route("/api/users/register", post(users::register_user))
    .route("/api/users/{id}", delete(users::delete_user))
      
    // Birthday routes
    .route("/api/birthdays/today", get(birthdays::get_todays_birthdays))
}