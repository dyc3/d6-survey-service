use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
pub fn register_user(user: Json<UserLoginParams>) -> Result<Json<UserToken>, Json<UserLoginError>> {
    let resp = UserToken {
        token: "token".to_string(),
    };
    Ok(Json(resp))
}
