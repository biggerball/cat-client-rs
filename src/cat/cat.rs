use std::sync::Arc;
use crate::Cat;
use crate::cat::aggregator_transaction::TransactionAggregator;
use crate::cat::config::Config;

use crate::message::transaction::Transaction;
use super::manager::CatMessageManager;
use super::scheduler::{self, Flush};
use super::sender::CatMessageSender;
use super::aggregator::CatLocalAggregator;



impl Cat {
    // pub fn new() -> Self {
    //     let config = Config::new();
    //
    //     let sender = Arc::new(CatMessageSender::new(&config));
    //     let manager = Arc::new(CatMessageManager::new(Arc::clone(&sender)));
    //     scheduler::background(Arc::clone(&manager) as Arc<CatMessageManager>);
    //     scheduler::background(Arc::clone(&sender) as Arc<CatMessageSender>);
    //
    //     let mut cat = Cat {
    //         manager: manager,
    //         sender: sender,
    //         aggregator: None
    //     };
    //
    //     let aggregator = Arc::new(CatLocalAggregator::new(&cat, Arc::clone(&sender)));
    //     cat.aggregator = Some(aggregator);
    //
    //     cat
    // }


    pub(crate) fn new_transaction_aggregator(&self, message_type: String, name: String) -> Transaction {
        Transaction::new(message_type, name, Some(Arc::clone(self.aggregator.get_flush_sedner())))
    }

    pub(crate) fn new_transaction_child(&self, message_type: String, name: String) -> Transaction {
        Transaction::new(message_type, name, None)
    }
}