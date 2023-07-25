use std::sync::Arc;
use async_std::channel;
use crate::message::message::{Message, MessageData, MessageGetter};

#[derive(Debug)]
pub struct Event {
    message_data: MessageData
}

impl  Event {
    pub fn new(message_type: String, name: String, flush_sender: Option<Arc<channel::Sender<Message>>>) -> Self {
        let message_data = MessageData::new(message_type, name, flush_sender);
        Event{
            message_data
        }
    }
}

impl MessageGetter for Event {
    fn get_type(&self) -> &String {
        self.message_data.get_type()
    }

    fn get_name(&self) -> &String {
        self.message_data.get_name()
    }

    fn get_status(&self) -> &'static str {
        self.message_data.get_status()
    }

    fn get_data(&self) -> &Vec<u8> {
        self.message_data.get_data()
    }

    fn get_time(&self) -> i64 {
        self.message_data.get_time()
    }
}