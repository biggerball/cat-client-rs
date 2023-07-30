use std::{sync::Mutex, cell::RefCell};

use crate::message::{transaction::Transaction, message::Message};

pub const TYPE_ID: &'static str = "DefaultTransaction";
pub struct DefaultTransaction {
    children: Mutex<RefCell<Vec<Message>>>,
}

impl DefaultTransaction {
    pub fn new() -> DefaultTransaction {
        DefaultTransaction { 
            children: Mutex::new(RefCell::new(Vec::new())) 
        }
    }
}

impl Transaction for DefaultTransaction {
    fn type_name(&self) -> &'static str {
        TYPE_ID
    }

    fn add_children(&self, message: Message) {
        if let Ok(guard) = self.children.lock() {
            let mut children = guard.borrow_mut();
            children.push(message);
        }
    }
}