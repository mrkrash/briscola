pub type RoomMapPointer = Arc<Mutex<RoomMap>>;

use crate::room::Room;
use rocket::futures::lock::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RoomMap(HashMap<String, Room>);

impl RoomMap {
    pub fn new() -> RoomMapPointer {
        Arc::new(Mutex::new(RoomMap(HashMap::new())))
    }

    pub fn get_room(&mut self, room_id: &String) -> &mut Room {
        let id = room_id.to_string();
        if !self.0.contains_key(&id) {
            self.0.insert(id.to_string(), Room::new(&id));
        }
        
        self.0.get_mut(&id).unwrap()
    }
}
