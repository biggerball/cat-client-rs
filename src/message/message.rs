use std::sync::{mpsc, Arc};

use crate::utils;

use super::{transaction::Transaction, consts};
use async_std::channel;

#[derive(Debug)]
pub struct MessageData<> {
    m_type: String,
    name: String,
    status: &'static str,
    timestamp: i64,
    data: Vec<u8>,
    flush_sender: Option<Arc<channel::Sender<Message>>>
}

pub trait MessageGetter {

    fn get_type(&self) -> &String;
    fn get_name(&self) -> &String;
    fn get_status(&self) -> &'static str;
    fn get_data(&self) -> &Vec<u8>;
    fn get_time(&self) -> i64;
}

impl MessageGetter for MessageData {
    fn get_type(&self) -> &String {
        &self.m_type
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_status(&self) -> &'static str {
        &self.get_status()
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn get_time(&self) -> i64 {
        self.timestamp
    }
}

impl  MessageData {
    pub fn new(m_type: String, name: String, flush_sender: Option<Arc<channel::Sender<Message>>>) -> Self {
        MessageData {
            m_type,
            name,
            status: consts::CAT_SUCCESS,
            timestamp: utils::get_timestamp(),
            data: Vec::new(),
            flush_sender,
        }
    }

    pub fn get_status(&self) -> &'static str {
        self.status
    }

    pub fn get_flush_sender(&self) -> &Option<Arc<channel::Sender<Message>>> {
        &self.flush_sender
    }
}

#[derive(Debug)]
pub enum Message {
    Transaction(Transaction),
    Event,
    Heartbeat,
}