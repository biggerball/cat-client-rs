use std::sync::Arc;
use crate::message::{transaction::Transaction, internal::{default_transaction::DefaultTransaction, null_transaction::NullTransaction}};

use super::manager::MessageManager;

pub struct MessageProducer {
    manager: Arc<MessageManager>,
    is_message_enable: bool
}

impl MessageProducer {
    pub fn new() -> MessageProducer {
        MessageProducer { 
            manager: Arc::new(MessageManager::new("domain".to_string(), "host".to_string(), "ip".to_string())), 
            is_message_enable: true 
        }
    }

    pub fn new_transaction(&self) -> Arc<dyn Transaction> {
        self.manager.set_up();
        if self.is_message_enable {
            let transaction = Arc::new(DefaultTransaction::new());
            self.manager.start(transaction.clone(), false);
            return transaction;
        } else {
            return Arc::new(NullTransaction{});
        }
    }
}