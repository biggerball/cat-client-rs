use crate::message::event::Event;
use crate::message::transaction::Transaction;


pub enum Message {
    Transaction(Box<dyn Transaction>),
    Event(Box<dyn Event>)
}