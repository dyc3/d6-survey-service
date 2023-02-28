use argon2::{Argon2, PasswordHasher};
use diesel::prelude::*;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordVerifier, SaltString};
use rocket::config::SecretKey;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api::ApiErrorResponse;
use crate::db::models::{ListedSurvey, NewUser, User};
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
    if user_params.username.is_empty() || user_params.password.is_empty() {
        return Err(UserLoginError::InvalidCredentials.into());
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user_params.password.as_bytes(), &salt)
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
    if user_params.username.is_empty() || user_params.password.is_empty() {
        return Err(UserLoginError::InvalidCredentials.into());
    }

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
                .verify_password(user_params.password.as_bytes(), &parsed_hash)
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

#[get("/user/surveys")]
pub async fn list_surveys(
    db: Storage,
    claims: Claims,
) -> Result<Json<Vec<ListedSurvey>>, ApiErrorResponse<UserLoginError>> {
    let surveys = db
        .run(move |conn| {
            schema::surveys::table
                .filter(schema::surveys::dsl::owner_id.eq(claims.user_id()))
                .select((
                    schema::surveys::dsl::id,
                    schema::surveys::dsl::title,
                    schema::surveys::dsl::description,
                    schema::surveys::dsl::published,
                    schema::surveys::dsl::owner_id,
                ))
                .load::<ListedSurvey>(conn)
        })
        .await
        .map_err(|e| {
            error!("{e:?}");
            UserLoginError::InternalError
        })?;
    Ok(Json(surveys))
}

#[cfg(test)]
mod tests {
    use rocket::local::blocking::Client;

    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn test_user_register() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let resp = client
                .post(uri!("/api", register_user))
                .body(r#"{"username": "a", "password": "a"}"#)
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::Created);
        })
    }

    #[test]
    fn test_user_login_valid() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let resp = client
                .post(uri!("/api", register_user))
                .body(r#"{"username": "a", "password": "a"}"#)
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::Created);

            let resp = client
                .post(uri!("/api", login_user))
                .body(r#"{"username": "a", "password": "a"}"#)
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::Ok);
        })
    }

    #[test]
    fn test_user_login_invalid() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let resp = client
                .post(uri!("/api", login_user))
                .body(r#"{"username": "a", "password": "a"}"#)
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::BadRequest);
        })
    }

    #[test]
    fn test_user_deny_register_blank_credentials() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let resp = client
                .post(uri!("/api", register_user))
                .body(r#"{"username": "a", "password": ""}"#)
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::BadRequest);

            let resp = client
                .post(uri!("/api", register_user))
                .body(r#"{"username": "", "password": "a"}"#)
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::BadRequest);
        })
    }

    #[test]
    fn test_user_deny_login_blank_credentials() {
        run_test_with_db(|db_name| {
            #[post("/make_invalid_users")]
            async fn make_invalid_users(db: Storage) {
                let users = vec![
                    UserLoginParams {
                        username: "a".to_string(),
                        password: "".to_string(),
                    },
                    UserLoginParams {
                        username: "".to_string(),
                        password: "a".to_string(),
                    },
                ];

                let argon2 = Argon2::default();
                let users = users
                    .iter()
                    .map(|user_params| {
                        let salt = SaltString::generate(&mut OsRng);
                        let password_hash = argon2
                            .hash_password(&user_params.password.as_bytes(), &salt)
                            .expect("valid password hash");
                        NewUser {
                            username: user_params.username.clone(),
                            password_hash: password_hash.to_string(),
                        }
                    })
                    .collect::<Vec<NewUser>>();

                db.run(move |conn| {
                    diesel::insert_into(schema::users::table)
                        .values(&users)
                        .execute(conn)
                })
                .await
                .expect("successful insert");
            }
            let client =
                Client::tracked(test_rocket(db_name).mount("/", routes![make_invalid_users]))
                    .expect("valid rocket instance");

            let resp = client.post(uri!(make_invalid_users)).dispatch();
            assert_eq!(resp.status(), rocket::http::Status::Ok);

            let resp = client
                .post(uri!("/api", login_user))
                .body(r#"{"username": "a", "password": ""}"#)
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::BadRequest);

            let resp = client
                .post(uri!("/api", login_user))
                .body(r#"{"username": "", "password": "a"}"#)
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::BadRequest);
        })
    }

    #[test]
    fn test_list_surveys_authorized() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            make_survey(&client, &token);
            make_survey(&client, &token);

            let resp = client
                .get(uri!("/api", list_surveys))
                .header(rocket::http::Header::new("Authorization", token))
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::Ok);
            let list = resp
                .into_json::<Vec<ListedSurvey>>()
                .expect("expected list of surveys");
            assert_eq!(list.len(), 2);
        })
    }

    #[test]
    fn test_list_surveys_only_for_self() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            make_survey(&client, &token);
            make_survey(&client, &token);

            let token2 = create_test_user(&client);
            make_survey(&client, &token2);

            let resp = client
                .get(uri!("/api", list_surveys))
                .header(rocket::http::Header::new("Authorization", token2))
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::Ok);
            let list = resp
                .into_json::<Vec<ListedSurvey>>()
                .expect("expected list of surveys");
            assert_eq!(list.len(), 1);
        })
    }
}
