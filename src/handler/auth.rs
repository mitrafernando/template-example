use crate::constant::KEYS;
use crate::helper::error::ResponseError;
use crate::interface::auth::{
    Claims, CreateUser, LoginUser, ResponseAuthorization, ResponseSignup,
};
use crate::repository::auth::{add_user, get_user_by_email};
use axum::{extract::Json as Req, response::Json, http::StatusCode};
use chrono::prelude::*;
use jsonwebtoken::{encode, Header};
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

pub async fn login(
    Req(payload): Req<LoginUser>,
) -> Result<Json<ResponseAuthorization>, ResponseError> {
    let email = payload.email;
    let password = payload.password.as_bytes();

    let user = get_user_by_email(email);

    let user_data = match user {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ResponseError::DataNotFound),
        Err(_) => return Err(ResponseError::DatabaseError),
    };

    let user_claims = Claims {
        aud: "me".to_owned(),
        sub: "b@b.com".to_owned(),
        user_id: user_data.id,
        fullname: user_data.fullname,
        email: user_data.email,
        exp: 10000000000,
    };

    let passowrd_hash = PasswordHash::new(&user_data.password).expect("invalid password hash");

    if Pbkdf2.verify_password(&password, &passowrd_hash).is_ok() {
        match encode(&Header::default(), &user_claims, &KEYS.encoding) {
            Ok(t) => {
                let res = ResponseAuthorization { access_token: t };
                Ok(Json(res))
            }
            Err(_) => Err(ResponseError::InternalError),
        }
    } else {
        Err(ResponseError::Unauthorized)
    }
}

pub async fn signup(Req(payload): Req<CreateUser>) -> Result<(StatusCode, Json<ResponseSignup>), ResponseError> {
    let fullname = payload.fullname;
    let email = payload.email;
    let email2 = email.clone();
    if let Some(_check_user) = get_user_by_email(email2).unwrap() {
        let res = ResponseSignup {
            status: 400,
            message: String::from("email already used, please use another email!"),
        };
        return Ok((StatusCode::BAD_REQUEST, Json(res)));
    }
    
    let password = payload.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Pbkdf2.hash_password(password, &salt).unwrap().to_string();
    let birth_place = payload.birth_place;
    let birth_date = NaiveDate::parse_from_str(&payload.birth_date, "%Y-%m-%d").unwrap();
    let gender = payload.gender;

    let user = add_user(
        fullname,
        email,
        password_hash,
        birth_place,
        birth_date,
        gender,
    );
    match user {
        Ok(Some(_user)) => {
            let res = ResponseSignup {
                status: 201,
                message: String::from("signup successfully"),
            };
            Ok((StatusCode::CREATED, Json(res)))
        }
        Ok(None) => Err(ResponseError::DataNotFound),
        Err(_) => Err(ResponseError::DatabaseError),
    }
}
