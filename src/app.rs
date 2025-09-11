use std::sync::Arc;
use crate::routes::create_routes;
use crate::services;

pub async fn run() {
  let db_pool = Arc::new(services::create_db_pool());

  let app = create_routes().with_state(db_pool);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}