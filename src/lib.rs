#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod api;
pub mod error;
pub mod models;
pub mod schema;
pub mod views;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
pub use error::MyError;
pub use models::User;
use std::env;
