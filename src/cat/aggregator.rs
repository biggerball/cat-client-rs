use std::sync::Arc;
use crate::Cat;
use crate::cat::consts::Signal;
use crate::cat::scheduler::{Flush, ScheduleMixer, ScheduleMixin};
use crate::message::message::Message;
use async_trait::async_trait;
use super::sender::CatMessageSender;
use async_std::channel;
use crate::cat::aggregator_transaction::TransactionAggregator;

pub const BATCH_FLAG: u8 = b'@';
pub const BATCH_SPLIT: u8 = b';';

pub struct CatLocalAggregator {
    flush_channel: (Arc<channel::Sender<Message>>, channel::Receiver<Message>),
    sender: Arc<CatMessageSender>
}

impl Flush for CatLocalAggregator {
    fn get_flush_sedner(&self) -> &Arc<channel::Sender<Message>> {
        &self.flush_channel.0
    }
}

#[async_trait::async_trait]
impl ScheduleMixer for CatLocalAggregator {
    fn get_name(&self) -> &'static str {
        "Aggregator"
    }

    async fn handle(&self, _: Signal) {
        todo!()
    }

    async fn process(&self) {
        tokio::select! {
            Ok(msg) = self.flush_channel.1.recv() => {
                if let Message::Transaction(_) = &msg {
                    self.sender.handle_transaction(msg).await;
                }
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
        todo!()
    }
}

impl CatLocalAggregator {
    pub fn new(sender: Arc<CatMessageSender>) -> CatLocalAggregator {
        let (message_sender, message_receiver) = channel::unbounded();
        CatLocalAggregator { 
            flush_channel: (Arc::new(message_sender), message_receiver),
            sender: sender
        }
    }
}