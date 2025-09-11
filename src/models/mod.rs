pub mod entities {
  pub mod people;
  pub mod cities;
  
  pub use people::*;
  pub use cities::*;
}

// Request modules
pub mod requests {
  pub mod people;
  pub mod cities;
  
  pub use cities::*;
}

// Response modules  
pub mod responses {
  pub mod birthday;
  pub mod cities;
  pub mod people;
  
  pub use cities::*;
}