# Planet Birthday API

A Rust-based REST API for managing users and their birthdays across different cities. Built with Axum and Diesel ORM for educational purposes.

## Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL
- **ORM**: Diesel
- **Async Runtime**: Tokio
- **Serialization**: Serde

## API Endpoints

### Cities
- `GET /api/cities` - Get all cities
- `POST /api/city` - Create a new city
- `DELETE /api/city` - Delete a city

### Users
- `GET /api/users` - Get all users
- `POST /api/users/register` - Register a new user
- `GET /api/users/{id}` - Get user by ID
- `DELETE /api/users/{id}` - Delete user
- `GET /api/users/city/{city_id}` - Get users by city

### Birthdays
- `GET /api/birthdays/today` - Get users with birthdays today

## Database Schema

### Cities Table
- `city_id` (Primary Key)
- `name`
- `country`
- `state` (Optional)
- `longitude`
- `latitude`

### Users Table
- `id` (Primary Key)
- `name`
- `birthday`
- `city_id` (Foreign Key)