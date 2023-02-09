use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use diesel::prelude::*;

use crate::db::{Storage, schema};
use crate::db::models::{User, NewUser};

#[typeshare]
#[derive(Clone, Serialize, Deserialize)]
pub struct UserLoginParams {
    username: String,
    password: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
) -> Result<Json<UserToken>, Json<UserLoginError>> {
    let user_params = user.into_inner();
    let user = NewUser {
        username: user_params.username,
        password: user_params.password,
    };
    db.run(move |conn| {
        diesel::insert_into(schema::users::table).values(&user).execute(conn)
    }).await.map_err(|e| {
        println!("{e:?}");
        UserLoginError::InternalError
    })?;

    let resp = UserToken {
        token: "token".to_string(),
    };
    Ok(Json(resp))
}

#[post("/user/login", data = "<user>")]
pub async fn login_user(
    db: Storage,
    user: Json<UserLoginParams>,
) -> Result<Json<UserToken>, Json<UserLoginError>> {
    let user_params = user.into_inner();

    db.run(move |conn| {
        use schema::users::dsl::*;
        let found_users: Vec<User> = schema::users::table.filter(username.eq(user_params.username)).load(conn).map_err(|e| {
            println!("{e:?}");
            UserLoginError::InternalError
        })?;
        if found_users.is_empty() {
            return Err(UserLoginError::InvalidCredentials);
        }
        Ok(())
    }).await?;

    let resp = UserToken {
        token: "token".to_string(),
    };
    Ok(Json(resp))
}
