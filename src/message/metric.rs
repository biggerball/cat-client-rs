use std::sync::Arc;
use crate::message::message::{Message, MessageData, MessageGetter, Messager};

#[derive(Debug)]
pub struct Metric {
    message_data: MessageData
}


impl Metric {
    pub fn new(message_type: String, name: String, flush_sender: Option<Arc<async_std::channel::Sender<Message>>>) -> Self {
        let message_data = MessageData::new(message_type, name, flush_sender);
        Metric{
            message_data
        }
    }

    pub async fn complete(self) {
        if let Some(flush) = &self.message_data.get_flush_sender() {
            let flush = Arc::clone(flush);
            flush.send(Message::Metric(self)).await.expect("err");
        }
    }
}

impl MessageGetter for Metric {
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

impl Messager for Metric {
    fn add_data_kv(&mut self, k: String, v: String) {
        todo!()
    }

    fn add_data_k(&mut self, k: String) {
        todo!()
    }

    fn set_data(&mut self, v: String) {
        todo!()
    }

    fn set_status(&mut self, status: &'static str) {
        todo!()
    }

    fn set_time(&mut self, time: i64) {
        todo!()
    }
}