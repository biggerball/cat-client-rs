use std::sync::Arc;
use crate::cat::consts::Signal;
use crate::cat::scheduler::{Flush, ScheduleMixer};
use crate::message::message::Message;
use async_trait::async_trait;
use super::sender::CatMessageSender;
use async_std::channel;

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

// #[async_trait]
// impl ScheduleMixer for CatLocalAggregator {
//     fn get_name(&self) -> &'static str {
//         "Aggregator"
//     }

//     async fn handle(&mut self, signal: Signal) {

//     }

//     async fn process(&mut self) {
//         tokio::select! {
//             Ok(msg) = self.flush_channel.1.recv() => {
//                 println!("flush agg: {:#?}", msg);
//             },
//         }
//     }

//     async fn after_start(&mut self) {

//     }

//     fn before_stop(&self) {

//     }

//     fn get_schedule_mixin(&self) {

//     }

//     fn set_active(&mut self, active: bool) {

//     }

//     fn is_active(&self) -> bool {
//         true
//     }
// }

impl CatLocalAggregator {
    pub fn new(sender: Arc<CatMessageSender>) -> CatLocalAggregator {
        let (message_sender, message_receiver) = channel::unbounded();
        CatLocalAggregator { 
            flush_channel: (Arc::new(message_sender), message_receiver),
            sender: sender
        }
    }
}