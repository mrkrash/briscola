use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}
