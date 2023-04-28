pub mod manager;
pub mod message;

use message::{PublicMessage, PrivateMessage};
use rocket::tokio::sync::broadcast::{channel, Sender, Receiver};
use std::collections::HashMap;

pub struct Room {
    _id: String,
    public_queue: Sender<PublicMessage>,
    private_queues: HashMap<String, Sender<PrivateMessage>>
}

impl Room {
    fn new(id: &String) -> Self {
        Self {
            _id: id.to_string(),
            public_queue: channel::<PublicMessage>(1024).0,
            private_queues: HashMap::new()
        }
    }

    pub fn get_private_receiver(&mut self, participant: &String) -> Receiver<PrivateMessage> {
        if !self.private_queues.contains_key(participant) {
            self.private_queues.insert(participant.to_string(), channel::<PrivateMessage>(1024).0);
        }
        self.private_queues.get(participant).unwrap().subscribe()
    }

    pub fn get_public_receiver(&self) -> Receiver<PublicMessage> {
        self.public_queue.subscribe()
    }

    pub fn add_participant(&self, participant: &String) {
        //TODO: handle participants state to prevent overcap, double enter, etc
        let _ = self.public_queue.send(PublicMessage {
            message: format!("{} entered room", participant)
        });
    }

    pub fn public_ping(&self) {
        let _ = self.public_queue.send(PublicMessage {
            message: "public ping".to_string()
        });
    }

    pub fn private_ping(&self) {
        for (participant, sender) in self.private_queues.iter() {
            let _result = sender.send(PrivateMessage {
                message: format!("private ping for {}", participant)
            });
        }
    }
}
