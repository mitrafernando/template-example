pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::{prelude::*, result::Error};
use dotenvy::dotenv;
use std::env;
use models::{Post, NewPost};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn create_post(title: &str, body: &str) -> Result<Option<Post>, Error> {
    use crate::schema::posts;
  
    let connection = &mut establish_connection();
  
    let new_post = NewPost { title, body };
  
    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(connection)
        .optional()
  }
