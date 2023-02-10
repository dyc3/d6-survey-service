use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents the claims in a JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: i32,
    /// Expiration time (as UTC timestamp)
    exp: u64,
}

impl Claims {
    pub fn new(user_id: i32) -> Self {
        let exp = jsonwebtoken::get_current_timestamp() + 2630000;
        Self { user_id, exp }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum JwtError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Expired token")]
    ExpiredToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = JwtError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(auth_header) = req.headers().get_one("Authorization") else {
            return Outcome::Failure((Status::Unauthorized, JwtError::InvalidToken));
        };
        if !auth_header.starts_with("Bearer") {
            return Outcome::Failure((Status::BadRequest, JwtError::InvalidToken));
        }
        let Some(token) = auth_header.split(" ").last() else {
            return Outcome::Failure((Status::BadRequest, JwtError::InvalidToken));
        };
        let key = req.rocket().config().secret_key.to_string();
        let key = jsonwebtoken::DecodingKey::from_secret(key.as_bytes());
        let validation = jsonwebtoken::Validation::default();

        let Ok(token_data) = jsonwebtoken::decode::<Claims>(token, &key, &validation) else {
            return Outcome::Failure((Status::Unauthorized, JwtError::InvalidToken));
        };

        if jsonwebtoken::get_current_timestamp() >= token_data.claims.exp {
            return Outcome::Failure((Status::Unauthorized, JwtError::ExpiredToken));
        }

        Outcome::Success(token_data.claims)
    }
}

mod tests {
    use jsonwebtoken::{DecodingKey, EncodingKey};
    use rocket::{http::Header, local::blocking::Client, Config};

    use super::*;

    #[test]
    fn user_id_types_match() {
        let claims = Claims { user_id: 1, exp: 1 };
        use crate::db::models::User;
        let user = User {
            id: 0,
            username: "".to_owned(),
            password_hash: "".to_owned(),
            created_at: chrono::NaiveDateTime::MIN,
            updated_at: chrono::NaiveDateTime::MAX,
        };
        // HACK: type_name_of_val is actually in the stdlib, but it's not stable yet.
        fn type_name_of_val<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let type_claim = format!("{}", type_name_of_val(claims.user_id));
        let type_user = format!("{}", type_name_of_val(user.id));
        assert_eq!(type_claim, type_user);
    }

    #[get("/")]
    fn test_get(claims: Claims) -> String {
        format!("Hello, {}!", claims.user_id)
    }

    #[launch]
    fn jwt_rocket() -> _ {
        rocket::build().mount("/", routes![test_get])
    }

    #[test]
    fn test_need_jwt() {
        let client = Client::tracked(jwt_rocket()).expect("valid rocket instance");

        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_accept_valid_jwt() {
        let client = Client::tracked(jwt_rocket()).expect("valid rocket instance");

        let key =
            EncodingKey::from_secret(client.rocket().config().secret_key.to_string().as_bytes());
        let claims = Claims {
            user_id: 1,
            exp: jsonwebtoken::get_current_timestamp() + 10000000,
        };
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key).unwrap();

        let mut req = client.get("/");
        req.add_header(Header::new("Authorization", format!("Bearer {token}")));
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_deny_malformed_header() {
        let client = Client::tracked(jwt_rocket()).expect("valid rocket instance");

        let key =
            EncodingKey::from_secret(client.rocket().config().secret_key.to_string().as_bytes());
        let claims = Claims {
            user_id: 1,
            exp: jsonwebtoken::get_current_timestamp() + 10000000,
        };
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key).unwrap();

        for h in [format!("{token}"), format!(" {token}")] {
            let mut req = client.get("/");
            req.add_header(Header::new("Authorization", h));
            let response = req.dispatch();
            assert_eq!(response.status(), Status::BadRequest);
        }
    }

    #[test]
    fn test_deny_expired_jwt() {
        let client = Client::tracked(jwt_rocket()).expect("valid rocket instance");

        let key =
            EncodingKey::from_secret(client.rocket().config().secret_key.to_string().as_bytes());
        let claims = Claims {
            user_id: 1,
            exp: jsonwebtoken::get_current_timestamp() - 200000,
        };
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key).unwrap();

        let mut req = client.get("/");
        req.add_header(Header::new("Authorization", format!("Bearer {token}")));
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }
}
