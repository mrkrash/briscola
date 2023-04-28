#[macro_use] extern crate rocket;

mod auth;
mod room;
mod user;

use auth::Token;
use dotenv::dotenv;
use rocket::{form::Form, State};
use rocket::fs::{relative, FileServer};
use rocket::response::stream::EventStream;
use room::map::{RoomMap, RoomMapPointer};
use user::User;

#[post("/authenticate", data = "<user>")]
fn authenticate(user: Form<User<'_>>) -> String {
    Token::create(user.username.to_string())
}

#[get("/room/<room>")]
async fn enter_room(
    room: String, token: Token, room_manager_pointer: &State<RoomMapPointer>
) -> String {
    room_manager_pointer.lock().await.get_room(&room).join(&token.username);
    "benvenuto".to_string()
}

#[get("/room/<room>/public")]
async fn room_public_channel(
    room: String, _token: Token, room_manager_pointer: &State<RoomMapPointer>
) -> EventStream![] {
    room_manager_pointer.lock().await.get_room(&room).queue.public_channel()
}

#[get("/room/<room>/private")]
async fn room_private_channel(
    room: String, token: Token, room_manager_pointer: &State<RoomMapPointer>
) -> EventStream![] {
    room_manager_pointer.lock().await.get_room(&room).queue.private_channel(&token.username)
}

#[get("/room/<room>/public-ping")]
async fn room_public_ping(
    room: String, _token: Token, room_manager_pointer: &State<RoomMapPointer>
) {
    room_manager_pointer.lock().await.get_room(&room).public_ping();
}

#[get("/room/<room>/private-ping")]
async fn room_private_ping(
    room: String, _token: Token, room_manager_pointer: &State<RoomMapPointer>
) {
    room_manager_pointer.lock().await.get_room(&room).private_ping();
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
    .manage(RoomMap::new())
    .mount("/", routes![
        authenticate,
        enter_room,
        room_public_channel,
        room_private_channel,
        room_public_ping,
        room_private_ping
    ])
    .mount("/", FileServer::from(relative!("static")))
}
