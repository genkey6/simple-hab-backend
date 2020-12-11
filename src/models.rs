use crate::db;
use crate::error::MyError;
use crate::schema::users;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
  id: i32,
  name: String,
}

impl User {
  pub fn find() -> Result<Vec<Self>, MyError> {
    let conn = db::connection()?;
    let users = users::table.load::<User>(&conn)?;
    Ok(users)
  }

  pub fn get(id: i32) -> Result<Self, MyError> {
    let conn = db::connection()?;
    let user = users::table.filter(users::id.eq(id)).first(&conn)?;
    Ok(user)
  }

  pub fn create(command: UserCommand) -> Result<Self, MyError> {
    let conn = db::connection()?;
    let user = diesel::insert_into(users::table)
        .values(command)
        .get_result(&conn)?;
    Ok(user)
  }

  pub fn update(id: i32, command:UserCommand) -> Result<Self, MyError> {
    let conn = db::connection()?;
    let user = diesel::update(users::table)
        .filter(users::id.eq(id))
        .set(command)
        .get_result(&conn)?;
    Ok(user)
  }

  pub fn delete(id: i32) -> Result<usize, MyError> {
    let conn = db::connection()?;
    let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&conn)?;
    Ok(res)
  }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct UserCommand {
  pub name: String,
}
