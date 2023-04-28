use rocket::serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub message: String
}
