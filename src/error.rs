use actix_web::ResponseError;
use thiserror::Error;

// Error をまとめる enum を定義する
// actix_web::ResponseError として使うために derive マクロで Debug を付与する必要がある
#[derive(Error, Debug)]
pub enum MyError {
  #[error("Failed to render HTML")]
  AskamaError(#[from] askama::Error),

  #[error("Failed to get connection")]
  ConnectionPoolError(#[from] r2d2::Error),

  #[error("Failed SQL execution")]
  SQLiteError(#[from] rusqlite::Error),
}

// actix_web::ResponseError を MyError に実装
impl ResponseError for MyError {}
