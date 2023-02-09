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
    use schema::users::dsl::*;
    db.run(move |conn| {
        diesel::insert_into(schema::users::table).values((username.eq(user.username), password.eq(user.password))).execute(conn)
    }).await.map_err(|e| {
        println!("{e:?}");
        UserLoginError::InternalError
    })?;

    let resp = UserToken {
        token: "token".to_string(),
    };
    Ok(Json(resp))
}
