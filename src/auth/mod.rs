mod claims;

use claims::Claims;
use jsonwebtoken::{
    encode, get_current_timestamp, EncodingKey,
};

pub struct Auth {
    claims: Claims
}

impl Auth {
    pub fn new(username: String) -> Self {
        Self {
            claims: Claims {
                sub: username,
                exp: get_current_timestamp()
            }
        }
    }

    pub fn token(&self) -> String {
        let token: Result<String, jsonwebtoken::errors::Error> = encode(
            &jsonwebtoken::Header::default(), 
            &self.claims, 
            &EncodingKey::from_secret("secret".as_ref())
        );
        token.unwrap()
    }
}
