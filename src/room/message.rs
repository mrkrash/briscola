use rocket::serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PublicMessage {
    pub message: String
}

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PrivateMessage {
    pub message: String
}