mod claims;

use claims::Claims;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, EncodingKey, Validation, DecodingKey,
};
use rocket::http::Status;
use rocket::{request::FromRequest, Request, request::Outcome};
use std::env;

#[derive(Debug)]
pub enum TokenError {
    MissingHeader,
    MissingBearer,
    Invalid,
}

pub struct Token {
    pub username: String
}

impl Token {
    pub fn create(username: String) -> String {
        let expiration_time = env::var(
            "JWT_EXPIRATION_TIME"
        ).unwrap_or("3600".to_string()).parse::<u64>().unwrap();

        let claims = Claims {
            sub: username,
            exp: get_current_timestamp() + expiration_time
        };

        let token: Result<String, jsonwebtoken::errors::Error> = encode(
            &jsonwebtoken::Header::default(), 
            &claims, 
            &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref())
        );

        token.unwrap()
    }

    fn validate(token: String) -> bool {
        let token = decode::<Claims>(
            &token, 
            &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()), 
            &Validation::default()
        );
        token.is_ok() && token.unwrap().claims.exp > get_current_timestamp()
    }

    fn bearer(authorization: &str) -> Option<&str> {
        if authorization.len() > 7 && authorization[..7] == "Bearer ".to_string() {
            return Some(authorization[7..].as_ref())
        }

        None
    }

    fn sub(token: &str) -> String {
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
            &Validation::default()
        );

        token.unwrap().claims.sub
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = request.headers().get_one("Authorization");
        if authorization.is_none() {
            return Outcome::Failure((Status::BadRequest, TokenError::MissingHeader))
        }

        match Token::bearer(authorization.unwrap()) {
            None => Outcome::Failure((Status::BadRequest, TokenError::MissingBearer)),
            Some(token) if Token::validate(token.to_string()) => Outcome::Success(Token {
                username: Token::sub(token)
            }),
            Some(_) => Outcome::Failure((Status::BadRequest, TokenError::Invalid)),
        }
    }
}
