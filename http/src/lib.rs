//! # Pteroxide HTTP
//!
//! `pteroxide-http` is the bindings library for HTTP requests to Pterodactyl for the Application
//! API and Client API.
//!
//! ## Examples
//!
//! Using the Application API:
//! ```
//! use pteroxide_http::application::App;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let app = App::new(
//!         "https://pterodactyl.domain".to_string(),
//!         "application_api_key".to_string(),
//!     );
//! 
//!     let user = app.get_user(4)
//!         .include("server")
//!         .exec()
//!         .await
//!         .expect("couldn't get the user");
//! 
//!     println!("{:#?}", user);
//! }
//! ```
//! 
//! Using the Client API:
//! ```
//! use pteroxide_http::client::Client;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new(
//!         "https://pterodactyl.domain".to_string(),
//!         "client_api_key".to_string(),
//!     );
//! 
//!     let acc = app.get_account()
//!         .exec()
//!         .await
//!         .expect("couldn't get the account");
//! 
//!     println!("{:#?}", acc);
//! }
//! ```

pub mod client;
pub mod errors;
pub mod requests;
