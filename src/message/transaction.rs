use std::sync::{mpsc, Arc};

use crate::utils;

use super::message::{MessageData, Message, MessageGetter};
use async_std::channel;

#[derive(Debug)]
pub struct Transaction {
    message_data: MessageData,
    children: Vec<Message>,
    duration_in_micros: i64,
    duration_start: i64
}

impl Transaction {
    pub fn new(m_type: String, name: String, flush_sender: Option<Arc<channel::Sender<Message>>>) -> Self {
        Transaction {
            message_data: MessageData::new(m_type, name, flush_sender) ,
            children: Vec::new(),
            duration_in_micros: 0,
            duration_start: 0,
        }
    }

    pub async fn complete(mut self) {
        self.duration_in_micros = utils::get_timestamp() - self.get_time();
        if let Some(flush) = &self.message_data.get_flush_sender() {
            let flush = Arc::clone(flush);
            flush.send(Message::Transaction(self)).await.expect("exp");
        }
    }
}

impl MessageGetter for Transaction {
    fn get_type(&self) -> &String {
        &self.message_data.get_type()
    }

    fn get_name(&self) -> &String {
        &self.message_data.get_name()
    }

    fn get_status(&self) -> &'static str {
        &self.message_data.get_status()
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.message_data.get_data()
    }

    fn get_time(&self) -> i64 {
        self.message_data.get_time()
    }
}