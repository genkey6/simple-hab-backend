use crate::models::{User, UserCommand};
use crate::MyError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/users")]
async fn find() -> Result<HttpResponse, MyError> {
  let users = User::find()?;
  Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn get(id: web::Path<i32>) -> Result<HttpResponse, MyError> {
  let user = User::get(id.into_inner())?;
  Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(command: web::Json<UserCommand>) -> Result<HttpResponse, MyError> {
  let user = User::create(command.into_inner())?;
  Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
async fn update(
  id: web::Path<i32>,
  command: web::Json<UserCommand>,
) -> Result<HttpResponse, MyError> {
  let user = User::update(id.into_inner(), command.into_inner())?;
  Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, MyError> {
  let deleted_user = User::delete(id.into_inner())?;
  Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_user })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(find);
  config.service(get);
  config.service(create);
  config.service(update);
  config.service(delete);
}
