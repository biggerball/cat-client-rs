use std::sync::Arc;

use super::{transaction::Transaction, event::Event};

pub enum Message {
    Transaction(Arc<dyn Transaction>),
    Event(Box<dyn Event>)
}