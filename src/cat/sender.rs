use std::sync::Mutex;
use async_std::channel;
use crate::cat::config::Config;
use crate::message::message::{Message, MessageGetter};
use super::scheduler::{ScheduleMixin};
use crate::cat::consts::Signal;
use crate::cat::scheduler::ScheduleMixer;
use crate::message::consts;
use crate::message::encoder_binary::BinaryEncoder;
use crate::message::header::Header;

pub struct CatMessageSender {
    schedule_mixin: ScheduleMixin,
    high: (channel::Sender<(Message, String)>, channel::Receiver<(Message, String)>),
    normal: (channel::Sender<(Message, String)>, channel::Receiver<(Message, String)>),
    buf: Mutex<Vec<u8>>,
    domain: String,
    hostname: String,
    ip: String
}


impl CatMessageSender {
    pub fn new(config: &Config) -> Self {
        CatMessageSender {
            schedule_mixin: ScheduleMixin::new(Signal::SignalSenderExit),
            high: channel::unbounded(),
            normal: channel::unbounded(),
            buf: Mutex::new(Vec::new()),
            domain: config.get_domain().clone(),
            hostname: config.get_hostname().clone(),
            ip: config.get_ip().clone(),
        }
    }
    pub async fn handle_transaction(&self, message_transaction: Message, id: String) {
        if let Message::Transaction(transaction) = &message_transaction {
            if transaction.get_status() != consts::CAT_SUCCESS {
                self.high.0.send((message_transaction, id)).await.expect("err");
            } else {
                self.normal.0.send((message_transaction, id)).await.expect("err");
            }
        }
    }

    pub async fn handle_event(&self, message_event: Message, id: String) {
        if let Message::Event(event) = &message_event {
            self.normal.0.send((message_event, id)).await.expect("err");
        }
    }

    fn send(&self, message: Message, id: String) {
        println!("send {:#?} {}", message, id);
        let mut buf = self.buf.lock().unwrap();
        buf.clear();

        let header = Header::new(self.domain.as_str(), self.hostname.as_str(), self.ip.as_str(), id);
        BinaryEncoder::encode_header(&mut *buf, &header);


    }
}

#[async_trait::async_trait]
impl ScheduleMixer for CatMessageSender {
    fn get_name(&self) -> &'static str {
        "Sender"
    }

    async fn process(&self) {
        tokio::select! {
            Ok((message, id)) = self.high.1.recv() => {
                self.send(message, id);
            },
            Ok((message, id)) = self.normal.1.recv() => {
                self.send(message, id);
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