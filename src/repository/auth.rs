use diesel::{prelude::*, result::Error};
use models::{NewUser, User};
use template_example::*;
use chrono::NaiveDate;

pub fn add_user(
    fullname: String,
    email: String,
    password: String,
    birth_place: String,
    birth_date: NaiveDate,
    gender: String,
) -> Result<Option<User>, Error> {
    use self::schema::users;

    let connection = &mut establish_connection();
    let new_user = NewUser {
        fullname,
        email,
        password,
        birth_place,
        birth_date,
        gender,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .optional()
}

pub fn get_user_by_email(user_email: String) -> Result<Option<User>, Error> {
    use self::schema::users::{dsl::users, email};

    let connection = &mut establish_connection();
    users
        .filter(email.eq(&user_email))
        .select(User::as_select())
        .first(connection)
        .optional()
}
