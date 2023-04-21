mod claims;

use claims::Claims;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, EncodingKey, Validation, DecodingKey,
};
use rocket::http::Status;
use rocket::{request::FromRequest, Request, request::Outcome};

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

pub struct Token { }

impl Token {
    pub fn create(username: String) -> String {
        let claims = Claims {
            sub: username,
            exp: get_current_timestamp() + 30
        };

        let token: Result<String, jsonwebtoken::errors::Error> = encode(
            &jsonwebtoken::Header::default(), 
            &claims, 
            &EncodingKey::from_secret("secret".as_ref())
        );
        
        token.unwrap()
    }

    fn validate(token: String) -> bool {
        let token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());
        token.is_ok() && token.unwrap().claims.exp > get_current_timestamp()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        match request.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, TokenError::Missing)),
            Some(token) if Token::validate(token.to_string()) => Outcome::Success(Token {}),
            Some(_) => Outcome::Failure((Status::BadRequest, TokenError::Invalid)),
        }
    }
}
