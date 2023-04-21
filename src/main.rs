#[macro_use] extern crate rocket;
mod auth;
mod user;

use user::User;
use auth::Auth;
use rocket::form::Form;

#[get("/")]
fn index() -> &'static str {
    "Ciao Giovini!"
}

#[post("/authenticate", data = "<user>")]
fn authenticate(user: Form<User<'_>>) -> String {
    Auth::new(user.username.to_string()).token()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, authenticate])
}
