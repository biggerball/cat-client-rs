use std::cell::{RefCell, Cell};
use std::sync::Arc;
use cat::aggregator_transaction::TransactionAggregator;
use cat::scheduler;
use message::consts;
use message::message::Messager;

use crate::cat::aggregator::CatLocalAggregator;
use crate::cat::aggregator_event::EventAggregator;
use crate::cat::config::Config;
use crate::cat::manager::CatMessageManager;
use crate::cat::scheduler::Flush;
use crate::cat::sender::CatMessageSender;
use crate::message::event::Event;
use crate::message::transaction::Transaction;

mod cat;
mod message;
mod utils;


pub struct Cat {
    manager: Arc<CatMessageManager>,
    sender: Arc<CatMessageSender>,
    aggregator: Arc<CatLocalAggregator>,
    transaction_aggregator: Arc<TransactionAggregator>,
    event_aggregator: Arc<EventAggregator>,
}

impl Cat {

    pub fn new(config: Config) -> Cat {
        let sender = Arc::new(CatMessageSender::new(&config));
        let aggregator = Arc::new(CatLocalAggregator::new(Arc::clone(&sender)));
        let transaction_aggregator = Arc::new(TransactionAggregator::new(Arc::clone(&aggregator)));
        let event_aggregator = Arc::new(EventAggregator::new(Arc::clone(&aggregator)));

        let manager = Arc::new(CatMessageManager::new(Arc::clone(&sender),Arc::clone(&transaction_aggregator), Arc::clone(&event_aggregator)));
        

        scheduler::background(Arc::clone(&manager) as Arc<CatMessageManager>);
        scheduler::background(Arc::clone(&sender) as Arc<CatMessageSender>);
        scheduler::background(Arc::clone(&transaction_aggregator) as Arc<TransactionAggregator>);
        // scheduler::background(Arc::clone(&event_aggregator) as Arc<EventAggregator>);

        let cat = Cat {
            manager,
            sender,
            aggregator,
            transaction_aggregator,
            event_aggregator
        };

        cat
    }

    pub fn new_transaction(&self, message_type: String, name: String) -> Transaction {
        Transaction::new(message_type, name, Some(Arc::clone(self.manager.get_flush_sedner())))
    }

    pub async fn log_error(&self, error: Box<dyn std::error::Error>) {
        let mut event = Event::new(String::from("Error"), String::from("CAT_ERROR"), Some(Arc::clone(self.manager.get_flush_sedner())));
        event.set_status(consts::CAT_ERROR);
        event.set_data(format!("Error:\n{:?}", error));
        event.complete().await;
    }

    pub async fn log_error_category(&self, error: Box<dyn std::error::Error>, category: String) {
        let mut event = Event::new(String::from("Error"), category, Some(Arc::clone(self.manager.get_flush_sedner())));
        event.set_status(consts::CAT_ERROR);
        event.set_data(format!("Error:\n{:?}", error));
        event.complete().await;
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

        let transaction = cat.new_transaction("message_type".to_string(), "name".to_string());
        transaction.complete().await;


        cat.log_error(produce_error().unwrap_err()).await;

        std::thread::sleep(Duration::from_secs(10));
    }

    fn produce_error() -> Result<(), Box<dyn std::error::Error>> {
        std::fs::read_to_string("produce error")?;
        Ok(())
    }
}