//! # Pteroxide Models
//! 
//! `pteroxide-models` implements all the object models for the Application API and Client API.
//! In most cases these models shouldn't be instantiated directly, but intstead be accessed as a
//! resulting of a method (e.g. from the `pteroxide-http` module).

pub mod client;
pub mod fractal;
pub mod global;
