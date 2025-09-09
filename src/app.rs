use crate::routes::create_routes;

pub async fn run() {

  let app = create_routes();

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}