use crate::room::message::Message;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::{select, sync::broadcast::{channel, error::RecvError, Sender}};
use std::collections::HashMap;

pub trait Queue {
    fn get_subs(&mut self) -> Vec<String>;

    fn send_private_message(&self, sub: &String, message: Message);

    fn send_public_message(&self, message: Message);
}

pub struct RoomQueue {
    public: Sender<Message>,
    privates: HashMap<String, Sender<Message>>
}

impl Queue for RoomQueue {
    fn get_subs(&mut self) -> Vec<String> {
        let mut keys = Vec::new();
        for (key, _) in self.privates.iter() {
            keys.push(key.to_string());
        }
        keys
    }

    fn send_private_message(&self, sub: &String, message: Message) {
        let _ = self.privates.get(sub).unwrap().send(message);
    }

    fn send_public_message(&self, message: Message) {
        let _ = self.public.send(message);
    }
}

impl RoomQueue {
    pub fn new() -> Self {
        Self {
            public: channel::<Message>(1024).0,
            privates: HashMap::new()
        }
    }

    fn channel(&self, sender: &Sender<Message>) -> EventStream![] {
        let mut receiver = sender.subscribe();
        EventStream! {
            loop {
                let msg = select! {
                    msg = receiver.recv() => match msg {
                        Ok(msg) => msg,
                        Err(RecvError::Closed) => break,
                        Err(RecvError::Lagged(_)) => continue,
                    }
                };
                yield Event::json(&msg);
            }
        }
    }

    pub fn private_channel(&mut self, sub: &String) -> EventStream![] {
        if !self.privates.contains_key(sub) {
            self.privates.insert(
                sub.to_string(), 
                channel::<Message>(1024).0
            );
        }
        self.channel(self.privates.get(sub).unwrap())
    }

    pub fn public_channel(&self) -> EventStream![] {
        self.channel(&self.public)
    }
}
