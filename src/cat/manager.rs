use std::cell::Cell;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};

use async_std::channel;
use crate::cat::consts::Signal;
use crate::cat::scheduler::ScheduleMixer;
use crate::message::consts;
use crate::message::message::{Message, MessageGetter};
use super::{sender::CatMessageSender, scheduler::{Flush, ScheduleMixin}};

pub struct CatMessageManager {
    schedule_mixin: ScheduleMixin,
    flush_channel: (Arc<channel::Sender<Message>>, channel::Receiver<Message>),
    sender: Arc<CatMessageSender>,
    index: AtomicU64,
    offset: Mutex<u32>,
    hour: Mutex<i32>,
    message_id_prefix: String
}

#[async_trait::async_trait]
impl ScheduleMixer for CatMessageManager {
    fn get_name(&self) -> &'static str {
        "Manager"
    }

    async fn process(&self) {
        tokio::select! {
            Ok(msg) = self.flush_channel.1.recv() => {
                println!("flush manager: {:#?}", msg);
                match &msg {
                    Message::Transaction(transaction) => {
                        let message_id = self.next_id();
                        if transaction.get_status() != consts::CAT_SUCCESS {
                            self.sender.handle_transaction(msg, message_id).await;
                        } else if true {
                            self.sender.handle_transaction(msg, message_id).await;
                        } else {

                        }
                    },
                    Message::Event => {

                    }
                    _ => {}
                }
            },
            Ok(signal) = self.schedule_mixin.signals.1.recv() => {
                self.schedule_mixin.handle(signal).await
            }
        }
    }

    fn before_stop(&self) {
        todo!()
    }

    fn get_schedule_mixin(&self) -> &ScheduleMixin {
        &self.schedule_mixin
    }
}


impl CatMessageManager {
    pub fn new(sender: Arc<CatMessageSender>) -> CatMessageManager {
        let (message_sender, message_receiver) = channel::unbounded();
        CatMessageManager {
            schedule_mixin: ScheduleMixin::new(Signal::SignalManagerExit), 
            flush_channel: (Arc::new(message_sender), message_receiver),
            sender,
            index: AtomicU64::new(0),
            offset: Mutex::new(0),
            hour: Mutex::new(0),
            message_id_prefix: "".to_string(),
        }
    }

    fn next_id(&self) -> String{
        let new_index = self.index.fetch_add(1, Ordering::SeqCst);
        return self.message_id_prefix.clone() + new_index.to_string().as_str();
    }
}

impl Flush for CatMessageManager {
    fn get_flush_sedner(&self) -> &Arc<channel::Sender<Message>> {
        &self.flush_channel.0
    }
}