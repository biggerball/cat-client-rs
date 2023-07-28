use std::{sync::{atomic::{AtomicI64, Ordering}, Arc, Mutex}, cell::RefCell, collections::HashMap};

use async_std::channel;
use crate::{cat::consts::Signal, message::{message::MessageGetter, self}};
use crate::cat::scheduler::Flush;
use crate::cat::consts;
use crate::cat::aggregator;

use crate::message::event::Event;
use crate::message::message::{Message, Messager};
use crate::message::transaction::Transaction;

use super::scheduler::{ScheduleMixin, ScheduleMixer};

struct EventData {
    m_type: String,
    name: String,
    count: AtomicI64,
    fail: AtomicI64,
}

pub struct EventAggregator {
    aggregator: Arc<aggregator::CatLocalAggregator>,
    schedule_mixin: ScheduleMixin,
    data_map: Mutex<RefCell<HashMap<String, EventData>>>,
    ch: (channel::Sender<Event>, channel::Receiver<Event>),
}

impl EventData {
    fn new(m_type: &String, name: &String) -> Self {
        EventData { 
            m_type: m_type.clone(),
            name: name.clone(),
            count: AtomicI64::new(0),
            fail: AtomicI64::new(0),
        }
    }

    fn add(&mut self, event: &Event) {
        self.count.fetch_add(1, Ordering::SeqCst);
        if event.get_status() != message::consts::CAT_SUCCESS {
            self.fail.fetch_add(1, Ordering::SeqCst);
        }
    }
}


impl EventAggregator {

    pub fn new(aggregator: Arc<aggregator::CatLocalAggregator>) -> Self {
        EventAggregator{
            aggregator,
            schedule_mixin: ScheduleMixin::new(consts::Signal::SignalSenderExit),
            data_map: Mutex::new(RefCell::new(Default::default())),
            ch: channel::unbounded()
        }
    }

    fn put_or_merge(&self, event: Event) {
        let data_map = self.data_map.lock().unwrap();
        let mut data_map = data_map.borrow_mut();

        let key = format!("{},{}", event.get_type(), event.get_name());
        if let Some(event_data) = data_map.get_mut(&key) {
            event_data.add(&event);
        } else {
            let mut new_event_data = EventData::new(event.get_type(), event.get_name());
            new_event_data.add(&event);
            data_map.insert(key, new_event_data);
        }
    }

    pub async fn handle_event(&self, event: Message) {
        if let Message::Event(event) = event {
            self.ch.0.send(event).await.expect("err");
        }
    }

    async fn send(&self, data_map: HashMap<String, EventData>) {
        if data_map.len() == 0 {
            return;
        }
        let mut transaction = Transaction::new(
            consts::TYPE_SYSTEM.to_string(),
            consts::NAME_TRANSACTION_AGGREGATOR.to_string(),
            Some(Arc::clone(self.aggregator.get_flush_sedner())));

        for (_, event_data) in data_map {
            if let Ok(event) = transaction.new_event(event_data.m_type, event_data.name) {
                event.add_data_k(format!("{}{}{}{}", aggregator::BATCH_FLAG, event_data.count.load(Ordering::SeqCst), aggregator::BATCH_FLAG, event_data.fail.load(Ordering::SeqCst)));
            }
        }
        transaction.complete().await;
    }
}


#[async_trait::async_trait]
impl ScheduleMixer for EventAggregator {
    fn get_name(&self) -> &'static str {
        "EventAggregator"
    }

    async fn handle(&self, _: Signal) {
        todo!()
    }

    async fn process(&self) {
        tokio::select! {
            Ok(event) = self.ch.1.recv() => {
                println!("event_aggre {:#?}", event);
                self.put_or_merge(event);
            },
            Ok(signal) = self.schedule_mixin.signals.1.recv() => {
                self.schedule_mixin.handle(signal).await
            }
        }
    }

    async fn after_start(&self) {
        todo!()
    }

    fn before_stop(&self) {
        todo!()
    }

    fn get_schedule_mixin(&self) -> &ScheduleMixin {
        &self.schedule_mixin
    }
}