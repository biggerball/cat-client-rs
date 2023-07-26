use std::sync::Arc;
use crate::cat::aggregator_transaction::TransactionAggregator;
use crate::cat::config::Config;

use crate::message::transaction::Transaction;
use super::manager::CatMessageManager;
use super::scheduler::{self, Flush};
use super::sender::CatMessageSender;
use super::aggregator::CatLocalAggregator;

pub struct Cat {
    manager: Arc<CatMessageManager>,
    sender: Arc<CatMessageSender>,
    aggregator: Arc<CatLocalAggregator>
}

impl Cat {
    pub fn new() -> Self {
        let config = Config::new();

        let sender = Arc::new(CatMessageSender::new(&config));
        let manager = Arc::new(CatMessageManager::new(Arc::clone(&sender)));
        let aggregator = Arc::new(CatLocalAggregator::new(Arc::clone(&sender)));

        scheduler::background(Arc::clone(&manager) as Arc<CatMessageManager>);
        scheduler::background(Arc::clone(&sender) as Arc<CatMessageSender>);

        let cat = Cat {
            manager: manager,
            sender: sender,
            aggregator: aggregator
        };
        let transaction_aggregator = Arc::new(TransactionAggregator::new(&cat));
        cat
    }

    pub fn new_transaction(&self, message_type: String, name: String) -> Transaction {
        Transaction::new(message_type, name, Some(Arc::clone(self.manager.get_flush_sedner())))
    }

    pub(crate) fn new_transaction_aggregator(&self, message_type: String, name: String) -> Transaction {
        Transaction::new(message_type, name, Some(Arc::clone(self.aggregator.get_flush_sedner())))
    }

    pub(crate) fn new_transaction_child(&self, message_type: String, name: String) -> Transaction {
        Transaction::new(message_type, name, None)
    }
}