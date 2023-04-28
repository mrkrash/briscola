pub mod map;
mod message;
mod queue;

use message::Message;
use queue::{Queue, RoomQueue};

pub struct Room {
    _id: String,
    pub queue: RoomQueue
}

impl Room {
    fn new(id: &String) -> Self {
        Self {
            _id: id.to_string(),
            queue: RoomQueue::new()
        }
    }

    pub fn join(&self, participant: &String) {
        let _ = &self.queue.send_public_message(Message {
            message: format!("{} joined room", participant)
        });
    }
}

// development purpose
impl Room {
    pub fn public_ping(&mut self) {
        let _ = &self.queue.send_public_message(Message {
            message: "public ping".to_string()
        });
    }

    pub fn private_ping(&mut self) {
        for sub in self.queue.get_subs().into_iter() {
            let _ = self.queue.send_private_message(&sub, Message {
                message: format!("private ping for {}", sub)
            });
        }
    }
}
