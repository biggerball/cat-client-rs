use std::cell::{RefCell, Cell};
use std::sync::Arc;

use crate::message::spi::context::Context;
use crate::message::transaction::Transaction;

pub struct MessageManager {
    domain: String,
    host: String,
    ip: String,
    m_first_message: Cell<bool>
}

impl MessageManager {
    thread_local! {
        static CONTEXT: RefCell<Option<Context>> = RefCell::new(None)
    }

    pub fn new(domain: String, host: String, ip: String) -> MessageManager {
        MessageManager { 
            domain, 
            host, 
            ip, 
            m_first_message: Cell::new(true) }
    }

    pub fn set_up(&self) {
        Self::CONTEXT.with(|context| {
            if let None = context.take() {
                println!("set up");
                context.replace(Some(Context::new(self.ip.clone(), self.host.clone(), self.domain.clone())));
            }
        });
    }

    pub fn start(&self, transaction: Arc<dyn Transaction>, forked: bool) {
        Self::CONTEXT.with(|context| {
            if let Some(context) = context.borrow_mut().as_mut() {
                context.start(transaction, forked);
            } else if self.m_first_message.get() {
                self.m_first_message.set(false);
            }
        });
    }
}