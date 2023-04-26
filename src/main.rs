#[macro_use] extern crate rocket;

mod auth;
mod user;

use auth::Token;
use dotenv::dotenv;
use rocket::form::Form;
use user::User;

#[get("/")]
fn index() -> &'static str {
    "Ciao Giovini!"
}

#[post("/authenticate", data = "<user>")]
fn authenticate(user: Form<User<'_>>) -> String {
    Token::create(user.username.to_string())
}

#[get("/room/<room>")]
fn room(room: &str, _token: Token) -> String {
    format!("Welcome to {}!", room)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index, authenticate, room])
}
