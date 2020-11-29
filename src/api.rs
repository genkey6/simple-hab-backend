use crate::views::{UserEntry, UsersTemplate};
use crate::MyError;
use actix_web::{get, http::header, post, web, HttpResponse};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddParams {
  name: String,
}

#[derive(Deserialize)]
pub struct DeleteParams {
  id: u32,
}

#[get("/users")]
pub async fn list(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
  let conn = db.get()?;
  let mut statement = conn.prepare("SELECT id, name FROM user")?;
  let rows = statement.query_map(params![], |row| {
    let id = row.get(0)?;
    let name = row.get(1)?;
    Ok(UserEntry { id, name })
  })?;

  let mut entries = Vec::new();
  for row in rows {
    entries.push(row?);
  }
  let html = UsersTemplate { entries };
  let response_body = html.render()?;
  Ok(
    HttpResponse::Ok()
      .content_type("text/html")
      .body(response_body),
  )
}

#[get("/user/{username}")]
pub async fn get_user() -> Result<HttpResponse, MyError> {
  let response_body = "Hello world!";

  // HttpResponse::Ok() はステータスコード 200 を持つ HttpResponseBuilder という構造体を返す
  // HttpResponseBuilder の Body() という関数にレスポンスのボディを渡すと HttpResponse が返ってくる
  // 戻り値が Result 型なので Ok で包む
  Ok(HttpResponse::Ok().body(response_body))
}

#[post("/create/user")]
pub async fn post_user(
  params: web::Form<AddParams>,
  db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
  let conn = db.get()?;
  conn.execute("INSERT INTO user (name) VALUES (?)", &[&params.name])?;
  Ok(
    HttpResponse::SeeOther()
      .header(header::LOCATION, "/users")
      .finish(),
  )
}

#[post("/delete/user")]
pub async fn delete_user(
  params: web::Form<DeleteParams>,
  db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
  let conn = db.get()?;
  conn.execute("DELETE FROM user WHERE id=?", &[&params.id])?;
  Ok(
    HttpResponse::SeeOther()
      .header(header::LOCATION, "/users")
      .finish(),
  )
}
