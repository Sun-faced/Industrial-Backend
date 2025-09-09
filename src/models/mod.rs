// Entity modules
pub mod entities {
  pub mod people;
  pub mod cities;
  
  pub use people::Person;
  pub use cities::City;
}

// Request modules
pub mod requests {
    pub mod people;
    pub mod cities;
    
    pub use people::*;
    pub use cities::*;
}

// Response modules  
pub mod responses {
    pub mod birthday;
    pub mod cities;
    
    pub use birthday::*;
    pub use cities::*;
}