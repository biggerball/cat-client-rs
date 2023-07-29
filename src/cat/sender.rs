use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use async_std::channel;
use bytes::BufMut;
use crate::cat::config::Config;
use crate::message::message::{Message, MessageGetter};
use super::scheduler::ScheduleMixin;
use crate::cat::consts::Signal;
use crate::cat::scheduler::ScheduleMixer;
use crate::message::consts;
use crate::message::encoder_binary::BinaryEncoder;
use crate::message::header::Header;

pub struct CatMessageSender {
    schedule_mixin: ScheduleMixin,
    manager: MessageManager,
    high: (channel::Sender<Message>, channel::Receiver<Message>),
    normal: (channel::Sender<Message>, channel::Receiver<Message>),
    buf: Mutex<Vec<u8>>,
    config: (String, String, String)
}


impl CatMessageSender {
    pub fn new(config: &Config) -> Self {
        CatMessageSender {
            schedule_mixin: ScheduleMixin::new(Signal::SignalSenderExit),
            manager: MessageManager::new(),
            high: channel::unbounded(),
            normal: channel::unbounded(),
            buf: Mutex::new(Vec::new()),
            config: (config.get_domain().clone(), config.get_hostname().clone(), config.get_ip().clone())
        }
    }
    pub async fn handle_transaction(&self, message_transaction: Message) {
        if let Message::Transaction(transaction) = &message_transaction {
            if transaction.get_status() != consts::CAT_SUCCESS {
                self.high.0.send(message_transaction).await.expect("err");
            } else {
                self.normal.0.send(message_transaction).await.expect("err");
            }
        }
    }

    pub async fn handle_event(&self, message_event: Message) {
        if let Message::Event(_) = &message_event {
            self.normal.0.send(message_event).await.expect("err");
        }
    }

    fn send(&self, message: Message) {
        println!("send {:#?}", message);
        let mut buf = self.buf.lock().unwrap();
        buf.clear();

        buf.put_u32(0);
        let header = self.get_header();
        BinaryEncoder::encode_header(&mut *buf, &header);
        BinaryEncoder::encode_message(&mut *buf, &message);
        let b : [u8; 4] = ((buf.len() - 4) as i32).to_be_bytes();
        buf[0] = b[0];
        buf[1] = b[1];
        buf[2] = b[2];
        buf[3] = b[3];

    }

    fn get_header(&self) -> Header {
        let next_id = self.manager.next_id();
        Header::new(self.config.0.as_str(), self.config.1.as_str(), self.config.2.as_str(), next_id)
    }
}

#[async_trait::async_trait]
impl ScheduleMixer for CatMessageSender {
    fn get_name(&self) -> &'static str {
        "Sender"
    }

    async fn process(&self) {
        tokio::select! {
            Ok(message) = self.high.1.recv() => {
                self.send(message);
            },
            Ok(message) = self.normal.1.recv() => {
                self.send(message);
            }
            Ok(signal) = self.schedule_mixin.signals.1.recv() => {
                self.schedule_mixin.handle(signal).await
            }
        }
    }

    async fn after_start(&self) {
        todo!()
    }

    fn before_stop(&self) {
        todo!()
    }

    fn get_schedule_mixin(&self) -> &ScheduleMixin {
        &self.schedule_mixin
    }
}

struct MessageManager {
    index: AtomicU64,
    message_id_prefix: String
}

impl MessageManager {
    fn new() -> MessageManager {
        MessageManager{
            index: Default::default(),
            message_id_prefix: "".to_string()
        }
    }

    fn next_id(&self) -> String{
        let new_index = self.index.fetch_add(1, Ordering::SeqCst);
        return self.message_id_prefix.clone() + new_index.to_string().as_str();
    }
}