#[macro_use] extern crate rocket;

mod auth;
mod room;
mod user;

use auth::Token;
use dotenv::dotenv;
use rocket::{form::Form, State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::{select, sync::broadcast::error::RecvError};
use room::manager::{RoomManager, RoomManagerPointer};
use user::User;

#[post("/authenticate", data = "<user>")]
fn authenticate(user: Form<User<'_>>) -> String {
    Token::create(user.username.to_string())
}

#[get("/room/<room>")]
async fn enter_room(room: String, token: Token, room_manager_pointer: &State<RoomManagerPointer>) -> String {
    let mut room_manager = room_manager_pointer.lock().await;
    room_manager.get_room(&room).add_participant(&token.username);
    "benvenuto".to_string()
}

#[get("/room/<room>/public")]
async fn room_public_channel(room: String, _token: Token, room_manager_pointer: &State<RoomManagerPointer>, mut end: Shutdown) -> EventStream![] {
    let mut room_manager = room_manager_pointer.lock().await;
    let mut public_receiver = room_manager.get_room(&room).get_public_receiver();

    EventStream! {
        loop {
            let msg = select! {
                msg = public_receiver.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}

#[get("/room/<room>/private")]
async fn room_private_channel(room: String, token: Token, room_manager_pointer: &State<RoomManagerPointer>, mut end: Shutdown) -> EventStream![] {
    let mut room_manager = room_manager_pointer.lock().await;
    let mut private_receiver = room_manager.get_room(&room).get_private_receiver(&token.username);

    EventStream! {
        loop {
            let msg = select! {
                msg = private_receiver.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}

#[get("/room/<room>/public-ping")]
async fn room_public_ping(room: String, _token: Token, room_manager_pointer: &State<RoomManagerPointer>) {
    let mut room_manager = room_manager_pointer.lock().await;
    room_manager.get_room(&room).public_ping();
}

#[get("/room/<room>/private-ping")]
async fn room_private_ping(room: String, _token: Token, room_manager_pointer: &State<RoomManagerPointer>) {
    let mut room_manager = room_manager_pointer.lock().await;
    room_manager.get_room(&room).private_ping();
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
    .manage(RoomManager::new())
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
