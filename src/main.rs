mod app;
mod routes;
mod models;
mod services;
mod schema;

#[tokio::main]
async fn main() {
    app::run().await;
}
