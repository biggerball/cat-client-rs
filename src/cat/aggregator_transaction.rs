use bytes::{BufMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};
use crate::cat::cat::Cat;
use crate::cat;
use crate::cat::aggregator;
use crate::cat::scheduler::{ScheduleMixer, ScheduleMixin};
use crate::message;
use crate::message::encoder_binary::BinaryEncoder;
use crate::message::message::{Message, MessageGetter, Messager};
use crate::message::transaction::Transaction;

struct TransactionData {
    m_type: String,
    name: String,
    count: AtomicI64,
    fail: AtomicI64,
    sum: AtomicI64,
    duration: HashMap<i64, i64>
}

pub struct TransactionAggregator<'cat> {
    cat: &'cat Cat,
    schedule_mixin: ScheduleMixin,
    data_map: Mutex<RefCell<HashMap<String, (String, TransactionData)>>>,
}

impl TransactionData {
    fn new(m_type: &String, name: &String) -> TransactionData {
        TransactionData{
            m_type: m_type.clone(),
            name: name.clone(),
            count: AtomicI64::new(0),
            fail: AtomicI64::new(0),
            sum: AtomicI64::new(0),
            duration: HashMap::new(),
        }
    }

    fn add(&mut self, transaction: &Transaction) {
        self.count.fetch_add(1, Ordering::SeqCst);
        if transaction.get_status() != message::consts::CAT_SUCCESS {
            self.fail.fetch_add(1, Ordering::SeqCst);
        }

        let millis = transaction.get_duration();
        self.sum.fetch_add(millis, Ordering::SeqCst);

        let duration = Self::compute_duration(millis);
        if let Some(data) = self.duration.get_mut(&duration) {
            *data = *data + duration;
        } else {
            self.duration.insert(duration, 1);
        }
    }

    fn compute_duration(duration_in_millis: i64) -> i64 {
        return if duration_in_millis < 1 {
            1
        } else if duration_in_millis < 20 {
            duration_in_millis
        } else if duration_in_millis < 200 {
            duration_in_millis - duration_in_millis % 5
        } else if duration_in_millis < 500 {
            duration_in_millis - duration_in_millis % 20
        } else if duration_in_millis < 2000 {
            duration_in_millis - duration_in_millis % 50
        } else if duration_in_millis < 20000 {
            duration_in_millis - duration_in_millis % 500
        } else if duration_in_millis < 1000000 {
            duration_in_millis - duration_in_millis % 10000
        } else {
            let mut dk = 524288;
            if duration_in_millis > 3600 * 1000 {
                dk = 3600 * 1000;
            } else {
                while dk < duration_in_millis {
                    dk <<= 1;
                }
            }
            dk
        }
    }
}

impl <'cat> TransactionAggregator<'cat> {

    pub fn new(cat: &'cat Cat) -> Self {
        TransactionAggregator{
            cat,
            schedule_mixin: ScheduleMixin::new(cat::consts::Signal::SignalSenderExit),
            data_map: Mutex::new(RefCell::new(Default::default())),
        }
    }

    pub fn put_or_merge(&self, transaction: Transaction, id: String) {
        let data_map = self.data_map.lock().unwrap();
        let mut data_map = data_map.borrow_mut();

        let key = format!("{}-{}", transaction.get_type(), transaction.get_name());
        if let Some((_, transaction_data)) = data_map.get_mut(&key) {
            transaction_data.add(&transaction);
        } else {
            let mut new_transaction_data = TransactionData::new(transaction.get_type(), transaction.get_name());
            new_transaction_data.add(&transaction);
            data_map.insert(key, (id, new_transaction_data));
        }
    }


    async fn collect_and_send(&self) {
        let guard = self.data_map.lock().unwrap();
        let data = guard.take();
        guard.replace(HashMap::new());
        //release lock
        drop(guard);
        //send data
        self.send(data).await;
    }

    async fn send(&self, data_map: HashMap<String, (String, TransactionData)>) {
        if data_map.len() == 0 {
            return;
        }
        let mut transaction = self.cat.new_transaction_aggregator(cat::consts::TYPE_SYSTEM.to_string(), cat::consts::NAME_TRANSACTION_AGGREGATOR.to_string());

        for (_, (id, data_transaction)) in data_map {
            let data = encode_transaction_data(&data_transaction);
            let mut trans = self.cat.new_transaction_child(data_transaction.m_type, data_transaction.name);
            trans.set_data(String::from_utf8(data).unwrap());
            transaction.add_children(Message::Transaction(trans));
        }
        transaction.complete().await;
    }
}

#[async_trait::async_trait]
impl <'cat> ScheduleMixer for TransactionAggregator<'cat> {
    fn get_name(&self) -> &'static str {
        todo!()
    }

    async fn handle(&self, _: cat::consts::Signal) {
        todo!()
    }

    async fn process(&self) {
        todo!()
    }

    async fn after_start(&self) {
        todo!()
    }

    fn before_stop(&self) {
        todo!()
    }

    fn get_schedule_mixin(&self) -> &ScheduleMixin {
        todo!()
    }
}


fn encode_transaction_data(data: &TransactionData) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.put_u8(aggregator::BATCH_FLAG);
    BinaryEncoder::write_string(&mut buf, data.count.load(Ordering::SeqCst).to_string().as_str());
    buf.put_u8(aggregator::BATCH_SPLIT);
    BinaryEncoder::write_string(&mut buf, data.fail.load(Ordering::SeqCst).to_string().as_str());
    buf.put_u8(aggregator::BATCH_SPLIT);
    BinaryEncoder::write_string(&mut buf, data.sum.load(Ordering::SeqCst).to_string().as_str());
    buf.put_u8(aggregator::BATCH_SPLIT);
    let mut i = 0;
    for (k, v) in &data.duration {
        if i > 0 {
            buf.put_u8(b'|');
        }
        BinaryEncoder::write_string(&mut buf, k.to_string().as_str());
        buf.put_u8(b',');
        BinaryEncoder::write_string(&mut buf, v.to_string().as_str());
        i = i + 1;
    }
    buf
}