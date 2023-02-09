use argon2::{Argon2, PasswordHasher};
use diesel::prelude::*;
use password_hash::rand_core::OsRng;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use password_hash::{PasswordHash, PasswordVerifier, SaltString};

use crate::db::models::{NewUser, User};
use crate::db::{schema, Storage};

#[typeshare]
#[derive(Clone, Serialize, Deserialize)]
pub struct UserLoginParams {
    username: String,
    password: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Responder)]
#[response(content_type = "json")]
pub struct UserToken {
    token: String,
}

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum UserLoginError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Internal error")]
    InternalError,
}

#[post("/user/register", data = "<user>")]
pub async fn register_user(
    db: Storage,
    user: Json<UserLoginParams>,
) -> (Status, Result<Json<UserToken>, Json<UserLoginError>>) {
    let (status, resp) = match create_user(db, user.into_inner()).await {
        Ok(token) => (Status::Created, Ok(Json(token))),
        Err(UserLoginError::InvalidCredentials) => (Status::BadRequest, Err(Json(UserLoginError::InvalidCredentials))),
        Err(UserLoginError::InternalError) => (Status::InternalServerError, Err(Json(UserLoginError::InternalError))),
    };

    (status, resp)
}

async fn create_user(db: Storage, user_params: UserLoginParams) -> Result<UserToken, UserLoginError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(&user_params.password.as_bytes(), &salt).map_err(|e| {
        println!("{e:?}");
        match e {
            password_hash::Error::Password => UserLoginError::InvalidCredentials,
            _ => UserLoginError::InternalError,
        }
    })?;
    let user = NewUser {
        username: user_params.username,
        password_hash: password_hash.to_string(),
    };

    db.run(move |conn| {
        diesel::insert_into(schema::users::table)
            .values(&user)
            .execute(conn)
    })
    .await
    .map_err(|e| {
        println!("{e:?}");
        UserLoginError::InternalError
    })?;

    let resp = UserToken {
        token: "token".to_string(),
    };
    Ok(resp)
}

#[post("/user/login", data = "<user>")]
pub async fn login_user(
    db: Storage,
    user: Json<UserLoginParams>,
) -> Result<Json<UserToken>, Json<UserLoginError>> {
    let user_params = user.into_inner();

    db.run(move |conn| {
        use schema::users::dsl::*;
        let found_users: Vec<User> = schema::users::table
            .filter(username.eq(user_params.username))
            .load(conn)
            .map_err(|e| {
                println!("{e:?}");
                UserLoginError::InternalError
            })?;
        if found_users.is_empty() {
            return Err(UserLoginError::InvalidCredentials);
        }
        Ok(())
    })
    .await?;

    let resp = UserToken {
        token: "token".to_string(),
    };
    Ok(Json(resp))
}