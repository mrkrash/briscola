#[macro_use] extern crate rocket;
mod auth;
mod user;

use user::User;
use auth::token::Token;
use rocket::form::Form;

#[get("/")]
fn index() -> &'static str {
    "Ciao Giovini!"
}

#[post("/authenticate", data = "<user>")]
fn authenticate(user: Form<User<'_>>) -> String {
    Token::create(user.username.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, authenticate])
}
