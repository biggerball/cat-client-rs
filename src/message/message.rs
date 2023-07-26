use std::sync::Arc;
use bytes::{BufMut};
use crate::utils;

use super::{transaction::Transaction, consts};
use async_std::channel;
use crate::message::event::Event;

#[derive(Debug)]
pub struct MessageData<> {
    m_type: String,
    name: String,
    status: &'static str,
    timestamp: i64,
    data: Vec<u8>,
    flush_sender: Option<Arc<channel::Sender<(Message)>>>
}

pub trait MessageGetter {

    fn get_type(&self) -> &String;
    fn get_name(&self) -> &String;
    fn get_status(&self) -> &'static str;
    fn get_data(&self) -> &Vec<u8>;
    fn get_time(&self) -> i64;
}

pub trait Messager {
    fn add_data_kv(&mut self, k: String, v: String);
    fn add_data_k(&mut self, k: String);
    fn set_data(&mut self, v: String);
    fn set_status(&mut self, status: &'static str);
    fn set_time(&mut self, time: i64);
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

impl Messager for MessageData {
    fn add_data_kv(&mut self, k: String, v: String) {
        if self.data.len() != 0 {
            self.data.put_u8(b'&');
        }
        self.data.put(k.as_bytes());
        self.data.put_u8(b'=');
        self.data.put(v.as_bytes());
    }

    fn add_data_k(&mut self, k: String) {
        if self.data.len() != 0 {
            self.data.put_u8(b'&');
        }
        self.data.put(k.as_bytes());
    }

    fn set_data(&mut self, v: String) {
        self.data.clear();
        self.data.put(v.as_bytes());
    }

    fn set_status(&mut self, status: &'static str) {
        self.status = status;
    }

    fn set_time(&mut self, time: i64) {
        todo!()
    }
}

impl MessageData {
    pub fn new(m_type: String, name: String, flush_sender: Option<Arc<channel::Sender<(Message)>>>) -> Self {
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

    pub fn get_flush_sender(&self) -> &Option<Arc<channel::Sender<(Message)>>> {
        &self.flush_sender
    }
}

#[derive(Debug)]
pub enum Message {
    Transaction(Transaction),
    Event(Event),
    Heartbeat,
}