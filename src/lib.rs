#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

pub mod api;
pub mod db;
pub mod error;
pub mod models;
pub mod schema;
pub mod views;

pub use error::MyError;
pub use models::User;
