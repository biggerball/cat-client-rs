use std::sync::Arc;

use crate::utils;

use super::{message::{MessageData, Message, MessageGetter}, event::Event};
use async_std::channel;
use crate::message::message::Messager;

#[derive(Debug)]
pub struct Transaction {
    message_data: MessageData,
    children: Vec<Message>,
    is_completed: bool,
    duration_in_micros: i64,
    duration_start: i64
}

impl Transaction {
    pub fn new(m_type: String, name: String, flush_sender: Option<Arc<channel::Sender<(Message)>>>) -> Self {
        Transaction {
            message_data: MessageData::new(m_type, name, flush_sender) ,
            children: Vec::new(),
            is_completed: false,
            duration_in_micros: 0,
            duration_start: 0,
        }
    }

    pub fn get_duration(&self) -> i64 {
        self.duration_in_micros
    }

    pub async fn complete(mut self) {
        self.is_completed = true;
        self.duration_in_micros = utils::get_timestamp() - self.get_time();
        if let Some(flush) = &self.message_data.get_flush_sender() {
            let flush = Arc::clone(flush);
            flush.send(Message::Transaction(self)).await.expect("err");
        }
    }

    pub fn add_children(&mut self, children: Message) {
        self.children.push(children);
    }

    pub fn get_children(&self) -> &Vec<Message> {
        &self.children
    }

    pub fn get_duration_in_micros(&self) -> i64 {
        self.duration_in_micros
    }

    pub fn new_event(&mut self, message_type: String, name: String) -> Result<&mut Event,()> {
        self.children.push(Message::Event(Event::new(message_type, name, None)));
        //i don't know how to better
        let index = self.children.len();
        if let Message::Event(event) = &mut self.children[index - 1] {
            return Ok(event);
        } else {
            Err(())
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

#[async_trait::async_trait]
impl Messager for Transaction {
    fn add_data_kv(&mut self, k: String, v: String) {
        self.message_data.add_data_kv(k, v);
    }

    fn add_data_k(&mut self, k: String) {
        self.message_data.add_data_k(k);
    }

    fn set_data(&mut self, v: String) {
        self.message_data.set_data(v);
    }

    fn set_status(&mut self, status: &'static str) {
        self.set_status(status);
    }

    fn set_time(&mut self, time: i64) {
        todo!()
    }
}