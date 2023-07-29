use std::{sync::{Arc, Mutex}, cell::RefCell, collections::HashMap};
use std::sync::atomic::{AtomicI64, Ordering};
use async_std::channel;

use crate::{cat::aggregator, message::metric::Metric};
use crate::cat::consts::Signal;
use crate::message::message::MessageGetter;

use super::scheduler::{ScheduleMixin, ScheduleMixer};

pub struct MetricData {
    m_type: String,
    name: String,
    count: i64,
    duration: i64,
}

pub struct AggregatorMetric {
    aggregator: Arc<aggregator::CatLocalAggregator>,
    schedule_mixin: ScheduleMixin,
    data_map: Mutex<RefCell<HashMap<String, MetricData>>>,
    ch: (channel::Sender<MetricData>, channel::Receiver<MetricData>),
}

impl AggregatorMetric {
    fn put_or_merge(&self, metric_data: MetricData) {
        let data_map = self.data_map.lock().unwrap();
        let mut data_map = data_map.borrow_mut();

        if let Some(event_data) = data_map.get_mut(&metric_data.name) {
            event_data.count = event_data.count  + metric_data.count;
            event_data.duration = event_data.duration  + metric_data.duration;
        } else {
            data_map.insert(metric_data.name.clone(), metric_data);
        }
    }
}

#[async_trait::async_trait]
impl ScheduleMixer for AggregatorMetric {
    fn get_name(&self) -> &'static str {
        "MetricAggregator"
    }

    async fn handle(&self, _: Signal) {
        todo!()
    }

    async fn process(&self) {
        tokio::select! {
            Ok(metric) = self.ch.1.recv() => {
                // println!("metric_aggre {:#?}", metric);
                // self.put_or_merge(metric);
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