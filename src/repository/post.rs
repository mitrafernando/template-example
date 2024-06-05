use diesel::{prelude::*, result::Error};
use template_example::*;
use models::{NewPost, Post};

pub fn get_all_post() -> Vec<Post> {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts")
}

pub fn get_post_by_id(id: i32) -> Result<Option<Post>, Error> {
    use self::schema::posts::dsl::posts;

    let connection = &mut establish_connection();

    posts
        .find(id)
        .select(Post::as_select())
        .first(connection)
        .optional()
}

pub fn create_post(title: &str, body: &str) -> Result<Option<Post>, Error> {
    use self::schema::posts;

    let connection = &mut establish_connection();

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(connection)
        .optional()
}

pub fn publish_post_by_id(id: i32) -> Result<Post, Error> {
    use self::schema::posts::dsl::{posts, published};

    let connection = &mut establish_connection();

    diesel::update(posts.find(id))
    .set(published.eq(true))
    .returning(Post::as_returning())
    .get_result(connection)
}

pub fn delete_post_by_id(target: String) -> usize {
    use self::schema::posts::{dsl::posts, title};

    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .unwrap()
}
