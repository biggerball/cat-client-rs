use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use async_std::channel;
use crate::cat::aggregator_event::EventAggregator;
use crate::cat::aggregator_transaction::TransactionAggregator;
use crate::cat::consts::Signal;
use crate::cat::scheduler::ScheduleMixer;
use crate::message::consts;
use crate::message::message::{Message, MessageGetter};
use super::{sender::CatMessageSender, scheduler::{Flush, ScheduleMixin}};

pub struct CatMessageManager {
    schedule_mixin: ScheduleMixin,
    flush_channel: (Arc<channel::Sender<Message>>, channel::Receiver<Message>),
    sender: Arc<CatMessageSender>,
    transaction_aggregator: Arc<TransactionAggregator>,
    event_aggregator: Arc<EventAggregator>,
    offset: Mutex<u32>,
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
                        if transaction.get_status() != consts::CAT_SUCCESS {
                            self.sender.handle_transaction(msg).await;
                        } else if false {
                            self.sender.handle_transaction(msg).await;
                        } else {
                            self.transaction_aggregator.handle_transaction(msg).await;
                        }
                    },
                    Message::Event(event) => {
                        if event.get_status() != consts::CAT_SUCCESS {
                            self.sender.handle_event(msg).await;
                        } else {
                            self.event_aggregator.handle_event(msg).await;
                        }
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
    pub fn new(sender: Arc<CatMessageSender>, transaction_aggregator: Arc<TransactionAggregator>, event_aggregator: Arc<EventAggregator>) -> CatMessageManager {
        let (message_sender, message_receiver) = channel::unbounded();
        CatMessageManager {
            schedule_mixin: ScheduleMixin::new(Signal::SignalManagerExit), 
            flush_channel: (Arc::new(message_sender), message_receiver),
            sender,
            transaction_aggregator,
            event_aggregator,
            offset: Mutex::new(0),
        }
    }
}

impl Flush for CatMessageManager {
    fn get_flush_sedner(&self) -> &Arc<channel::Sender<Message>> {
        &self.flush_channel.0
    }
}