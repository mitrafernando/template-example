use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{posts, users};
use chrono::NaiveDate;

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
  pub title: &'a str,
  pub body: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: i32,
  pub fullname: String,
  pub email: String,
  pub password: String,
  pub birth_place: String,
  pub birth_date: NaiveDate,
  pub gender: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
  pub fullname: String,
  pub email: String,
  pub password: String,
  pub birth_place: String,
  pub birth_date: NaiveDate,
  pub gender: String,
}