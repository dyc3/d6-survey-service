use argon2::{Argon2, PasswordHasher};
use diesel::prelude::*;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordVerifier, SaltString};
use rocket::config::SecretKey;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{Orbit, Rocket};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api::ApiErrorResponse;
use crate::db::models::{NewUser, User};
use crate::db::{schema, Storage};
use crate::jwt::Claims;

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
    pub(crate) token: String,
}

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum UserLoginError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Internal error")]
    InternalError,
}

impl From<UserLoginError> for ApiErrorResponse<UserLoginError> {
    fn from(value: UserLoginError) -> Self {
        let status = match &value {
            UserLoginError::InvalidCredentials => Status::BadRequest,
            UserLoginError::InternalError => Status::InternalServerError,
        };
        ApiErrorResponse {
            status,
            message: value,
        }
    }
}

#[post("/user/register", data = "<user>")]
pub async fn register_user(
    db: Storage,
    user: Json<UserLoginParams>,
    secret: &SecretKey,
) -> Result<Created<Json<UserToken>>, ApiErrorResponse<UserLoginError>> {
    let user_params = user.into_inner();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(&user_params.password.as_bytes(), &salt)
        .map_err(|e| {
            error!("{e:?}");
            match e {
                password_hash::Error::Password => UserLoginError::InvalidCredentials,
                _ => UserLoginError::InternalError,
            }
        })?;
    let user = NewUser {
        username: user_params.username,
        password_hash: password_hash.to_string(),
    };

    let users = db
        .run(move |conn| {
            diesel::insert_into(schema::users::table)
                .values(&user)
                .get_results::<User>(conn)
        })
        .await
        .map_err(|e| {
            error!("{e:?}");
            UserLoginError::InternalError
        })?;
    let Some(user) = users.first() else {
        return Err(UserLoginError::InternalError.into());
    };
    let token = generate_jwt_for_user(secret, user.id).map_err(|e| {
        error!("{e:?}");
        UserLoginError::InternalError
    })?;

    let resp = UserToken { token };
    Ok(Created::new("").body(Json(resp)))
}

#[post("/user/login", data = "<user>")]
pub async fn login_user(
    db: Storage,
    user: Json<UserLoginParams>,
    secret: &SecretKey,
) -> Result<Json<UserToken>, ApiErrorResponse<UserLoginError>> {
    let user_params = user.into_inner();

    let user_id = db
        .run(move |conn| {
            use schema::users::dsl::*;
            let found_users: Vec<User> = schema::users::table
                .filter(username.eq(user_params.username))
                .load(conn)
                .map_err(|e| {
                    error!("{e:?}");
                    UserLoginError::InternalError
                })?;
            if found_users.is_empty() {
                return Err(UserLoginError::InvalidCredentials);
            }
            let Some(user) = found_users.first() else {
            return Err(UserLoginError::InternalError);
        };
            let parsed_hash =
                PasswordHash::new(user.password_hash.as_str()).map_err(|e| match e {
                    ::password_hash::Error::Password => UserLoginError::InvalidCredentials,
                    _ => UserLoginError::InternalError,
                })?;
            Argon2::default()
                .verify_password(&user_params.password.as_bytes(), &parsed_hash)
                .map_err(|e| match e {
                    ::password_hash::Error::Password => UserLoginError::InvalidCredentials,
                    _ => UserLoginError::InternalError,
                })?;
            Ok(user.id)
        })
        .await?;
    let token = generate_jwt_for_user(secret, user_id).map_err(|e| {
        error!("{e:?}");
        UserLoginError::InternalError
    })?;

    let resp = UserToken { token };
    Ok(Json(resp))
}

fn generate_jwt_for_user(secret: &SecretKey, user_id: i32) -> anyhow::Result<String> {
    let claims = Claims::new(user_id);
    let key = jsonwebtoken::EncodingKey::from_secret(secret.to_string().as_bytes());
    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key)?;
    Ok(token)
}
