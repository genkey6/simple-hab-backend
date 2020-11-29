// use crate::{Database, MyError, User};
// use actix_web::HttpResponse;

// pub async fn list_users_handler(db: Database) -> Result<HttpResponse, actix_web::Error> {
//   let db = db.lock().await;
//   let users = db
//     .clone()
//     .into_iter()
//     .map(|(_, v)| v)
//     .collect::<Vec<User>>();
//   Ok(HttpResponse::Ok().body(users))
// }

// pub async fn get_user_handler(db: Database, id: u64) -> Result<HttpResponse, actix_web::Error> {
//   let db = db.lock().await;
//   let user = db.get(&id);
//   match user {
//     None => Err(),
//     Some(u) => Ok(HttpResponse::Ok().body(user)),
//   }
// }

// pub async fn put_user_handler(
//   db: Database,
//   id: u64,
//   user: User,
// ) -> Result<HttpResponse, actix_web::Error> {
//   if id != user.id() {
//     return Ok();
//   };
//   let mut db = db.lock().await;
//   db.insert(user.id(), user.clone());
//   Ok()
// }
