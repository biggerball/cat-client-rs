use std::cell::RefCell;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicU32;
use std::{collections::VecDeque, sync::Arc};

use crate::message::message::Message;
use crate::message::transaction::Transaction;
use crate::message::internal::default_forked_transaction;

use super::message_tree::MessageTree;

pub struct Context {
    m_tree: MessageTree,
    m_stack: RefCell<VecDeque<Arc<dyn Transaction>>>,
    m_length: AtomicU32
}

impl Context {
    pub fn new(ip: String, host: String, domain: String) -> Context {
        Context { 
            m_tree: MessageTree::new(ip, host, domain), 
            m_stack: RefCell::new(VecDeque::new()),
            m_length: AtomicU32::new(0)
        }
    }

    pub fn start(&self, transaction: Arc<dyn Transaction>, forked: bool) {
        let mut m_stack = self.m_stack.borrow_mut();
        if m_stack.len() != 0 {
            if transaction.type_name() != default_forked_transaction::TYPE_ID {
                let parent = m_stack.back_mut();
                self.add_transaction_child(Message::Transaction(transaction.clone()), parent)
            }
        } else {
            self.m_tree.set_message(Message::Transaction(transaction.clone()));
        }
        if !forked {
            m_stack.push_back(transaction)
        }
    }

    fn add_transaction_child(&self, message: Message, transaction: Option<&mut Arc<dyn Transaction>>) {
        if let Some(transaction) = transaction {
            transaction.add_children(message);
            self.m_length.fetch_add(1, Ordering::SeqCst);
        }
    }
}