pub mod api;
mod db;
mod error;
mod handlers;
mod models;
pub mod views;

pub use db::{init_db, Database};
pub use error::MyError;
pub use models::User;
