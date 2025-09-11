use std::env;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;

pub mod cities;
pub mod birthdays;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> DbPool {
  dotenv().ok();
  
  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  
  let manager = ConnectionManager::<PgConnection>::new(&database_url);
  
  Pool::builder()
    .max_size(7)
    .build(manager)
    .expect("Failed to create connection pool")
}