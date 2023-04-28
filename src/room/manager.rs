pub type RoomManagerPointer = Arc<Mutex<RoomManager>>;

use crate::room::Room;
use rocket::futures::lock::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RoomManager {
    rooms: HashMap<String, Room>
}

impl RoomManager {
    pub fn new() -> RoomManagerPointer {
        Arc::new(Mutex::new(RoomManager {
            rooms: HashMap::new()
        }))
    }

    pub fn get_room(&mut self, room_id: &String) -> &mut Room {
        let id = room_id.to_string();
        if !self.rooms.contains_key(&id) {
            self.rooms.insert(id.to_string(), Room::new(&id));
        }
        
        self.rooms.get_mut(&id).unwrap()
    }
}
