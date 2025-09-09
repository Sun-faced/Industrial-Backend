mod app;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    app::run().await;
}
