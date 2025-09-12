-- Create the city table first (since users will reference it)
CREATE TABLE city (
  city_id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  country VARCHAR(100) NOT NULL,
  state VARCHAR(100),
  longitude DOUBLE PRECISION,
  latitude DOUBLE PRECISION
);

-- Create the users table with foreign key reference to city
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  birthday DATE,
  city_id INTEGER,
  FOREIGN KEY (city_id) REFERENCES city(city_id) ON DELETE SET NULL
);

-- Insert some sample cities
INSERT INTO city (name, country, state, longitude, latitude) VALUES 
('New York', 'United States', 'New York', -74.0059, 40.7128),
('London', 'United Kingdom', NULL, -0.1276, 51.5074),
('Tokyo', 'Japan', NULL, 139.6917, 35.6895),
('Toronto', 'Canada', 'Ontario', -79.3832, 43.6532),
('Sydney', 'Australia', 'New South Wales', 151.2093, -33.8688);


INSERT INTO users (name, birthday, city_id) VALUES 
('John Doe', '1990-05-15', 1),
('Jane Smith', '1985-12-03', 2),
('Mike Johnson', '1992-08-22', 1),
('Sarah Wilson', '1988-03-10', 3),
('David Brown', '1995-11-07', 4);
