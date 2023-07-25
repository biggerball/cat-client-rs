use bytes::{BufMut};
use crate::message::consts;
use crate::message::header::Header;
// use crate::message::message_event::Event;
use crate::message::message::{Message, MessageGetter};
// use crate::message::message_header::Header;
// use crate::message::message_heartbeat::HeartBeat;
// use crate::message::message_metric::Metric;
// use crate::message::message_trace::Trace;

const DEFAULT_THREAD_GROUP_NAME: &str = "";
const DEFAULT_THREAD_ID: &str = "0";
const DEFAULT_THREAD_NAME: &str = "";
const DEFAULT_SESSION_TOKEN: &str = "";

pub struct BinaryEncoder {}

impl BinaryEncoder {

    // pub fn encode_message(buf: &mut Vec<u8>, message: &Message) {
    //     match message {
    //         Message::Transaction(transaction) => {
    //             buf.put_u8(b't');
    //             Self::encode_message_start(buf, transaction);
    //             let children = transaction.get_children();
    //             for child in children {
    //                 Self::encode_message(buf, child);
    //             }
    //             buf.put_u8(b'T');
    //             Self::encode_message_end(buf, transaction);
    //             Self::write_i64(buf, transaction.get_duration_in_micros());
    //         }
    //         Message::Event(event) => {
    //             Self::encode_event(buf, event);
    //         }
    //         Message::Metric(metric) => {
    //             Self::encode_metric(buf, metric);
    //         }
    //         Message::Heartbeat(heartbeat) => {
    //             Self::encode_heartbeat(buf, heartbeat);
    //         }
    //         Message::Trace(trace) => {
    //             Self::encode_trace(buf, trace);
    //         }
    //     }
    // }

    pub fn encode_header(buf: &mut Vec<u8>, header: &Header) {
        Self::write_version(buf, consts::BINARY_PROTOCOL);
        Self::write_string(buf, header.get_domain());
        Self::write_string(buf, header.get_hostname());
        Self::write_string(buf, header.get_ip());

        Self::write_string(buf, DEFAULT_THREAD_GROUP_NAME);
        Self::write_string(buf, DEFAULT_THREAD_ID);
        Self::write_string(buf, DEFAULT_THREAD_NAME);

        Self::write_string(buf, header.get_message_id());
        Self::write_string(buf, header.get_parent_message_id());
        Self::write_string(buf, header.get_root_message_id());
        Self::write_string(buf, DEFAULT_SESSION_TOKEN);
    }

    // pub fn encode_event(buf: &mut Vec<u8>, event: &Event) {
    //     Self::encode_message_with_leader(buf, event, b'H');
    // }
    //
    // pub fn encode_heartbeat(buf: &mut Vec<u8>, heartbeat: &HeartBeat) {
    //     Self::encode_message_with_leader(buf, heartbeat, b'H');
    // }
    //
    // pub fn encode_metric(buf: &mut Vec<u8>, metric: &Metric) {
    //     Self::encode_message_with_leader(buf, metric, b'M');
    // }
    //
    // pub fn encode_trace(buf: &mut Vec<u8>, trace: &Trace) {
    //     Self::encode_message_with_leader(buf, trace, b'L');
    // }

    fn write_i64(buf: &mut Vec<u8>, i: i64) {
        let mut i = i;
        loop {
            if i & !0x7F == 0 {
                buf.put_u8(i as u8);
                return;
            } else {
                buf.put_u8((i & 0x7F | 0x80) as u8);
                i >>= 7;
            }
        }
    }

    fn write_string(buf: &mut Vec<u8>, s: &str) {
        if s.len() == 0 {
            Self::write_i64(buf, 0 as i64);
        } else {
            Self::write_i64(buf, s.as_bytes().len() as i64);
            buf.put(s.as_bytes());
        }
    }

    fn encode_message_start(buf: &mut Vec<u8>, message: &dyn MessageGetter) {
        Self::write_i64(buf, message.get_time());
        Self::write_string(buf, message.get_type());
        Self::write_string(buf, message.get_name());
    }

    fn encode_message_end(buf: &mut Vec<u8>, message: &dyn MessageGetter) {
        Self::write_string(buf, message.get_status());
        if message.get_data().len() == 0 {
            Self::write_i64(buf, 0 as i64);
        } else {
            Self::write_i64(buf, message.get_data().len() as i64);
            buf.put_slice(message.get_data());
        }
    }


    fn encode_message_with_leader(buf: &mut Vec<u8>, message: &dyn MessageGetter, leader: u8) {
        buf.put_u8(leader);
        Self::encode_message_start(buf, message);
        Self::encode_message_end(buf, message);
    }

    fn write_version(buf: &mut Vec<u8>, s: &str) {
        buf.put(s.as_bytes());
    }
}