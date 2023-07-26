use std::sync::Arc;
use cat::aggregator_transaction::TransactionAggregator;
use cat::scheduler;

use crate::cat::aggregator::CatLocalAggregator;
use crate::cat::config::Config;
use crate::cat::manager::CatMessageManager;
use crate::cat::scheduler::Flush;
use crate::cat::sender::CatMessageSender;
use crate::message::transaction::Transaction;

mod cat;
mod message;
mod utils;


pub struct Cat {
    manager: Arc<CatMessageManager>,
    sender: Arc<CatMessageSender>,
    aggregator: Arc<CatLocalAggregator>,
    transaction_aggregator: Arc<TransactionAggregator>
}

impl Cat {

    pub fn new(config: Config) -> Cat {
        let sender = Arc::new(CatMessageSender::new(&config));
        let manager = Arc::new(CatMessageManager::new(Arc::clone(&sender)));
        let aggregator = Arc::new(CatLocalAggregator::new(Arc::clone(&sender)));
        let transaction_aggregator = Arc::new(TransactionAggregator::new(Arc::clone(&aggregator)));
        

        scheduler::background(Arc::clone(&manager) as Arc<CatMessageManager>);
        scheduler::background(Arc::clone(&sender) as Arc<CatMessageSender>);
        let cat = Cat {
            manager,
            sender,
            aggregator,
            transaction_aggregator
        };

        cat
    }

    pub fn new_transaction(&self, message_type: String, name: String) -> Transaction {
        Transaction::new(message_type, name, Some(Arc::clone(self.manager.get_flush_sedner())))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[async_std::test]
    async fn test() {
        let config = Config::new();
        let cat = Cat::new(config);

        let mut transaction = cat.new_transaction("message_type".to_string(), "name".to_string());
        transaction.complete().await;
        std::thread::sleep(Duration::from_secs(10));
    }
}