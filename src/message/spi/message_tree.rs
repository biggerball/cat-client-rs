use std::sync::Mutex;

use crate::message::message::Message;

pub struct MessageTree{
    m_ip: String,
    m_host: String,
    m_domain: String,
    m_message: Mutex<Option<Message>>
}

impl MessageTree {
    pub fn new(ip: String, host: String, domain: String) -> MessageTree {
        MessageTree{
            m_ip: ip,
            m_host: host,
            m_domain: domain,
            m_message: Mutex::new(None)
        }
    }

    pub fn set_message(&self, message: Message) {
        let mut m_message = self.m_message.lock().unwrap();
        *m_message = Some(message);
    }

}