diesel::table! {
  city (city_id) {
    city_id -> Int4,
    name -> Varchar,
    country -> Varchar,
    state -> Nullable<Varchar>,
    longitude -> Float8,
    latitude -> Float8,
  }
}
diesel::table! {
  users (id) {
    id -> Int4,
    name -> Varchar,
    birthday -> Date,
    city_id -> Int4,
  }
}

diesel::joinable!(users -> city (city_id));

diesel::allow_tables_to_appear_in_same_query!(
  city,
  users,
);