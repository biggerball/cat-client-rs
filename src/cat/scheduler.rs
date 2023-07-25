use std::{sync::{Arc, Mutex}, cell::Cell};
use crate::message::message::Message;
use async_std::{channel, task};

use super::consts::Signal;

pub struct ScheduleMixin {
    pub is_alive: Arc<Mutex<Cell<bool>>>,
    pub exit_signal: Signal,
    pub signals: (channel::Sender<Signal>, channel::Receiver<Signal>)
}

#[async_trait::async_trait]
impl ScheduleMixer for ScheduleMixin {

    async fn handle(&self, signal: Signal) {
        if let Signal::SignalShutdown = signal {
            let is_alive = self.is_alive.lock().unwrap();
            is_alive.set(false);
        }
    }

    fn get_schedule_mixin(&self) -> &ScheduleMixin {
        panic!("unreachable")
    }
}

impl ScheduleMixin {
    pub fn new(exit_signal: Signal) -> Self {
        ScheduleMixin { 
            is_alive: Arc::new(Mutex::new(Cell::new(false))),
            signals: channel::unbounded(),
            exit_signal
        }
    }
}


#[async_trait::async_trait]
pub trait ScheduleMixer: Send + Sync {
    fn get_name(&self) -> &'static str {
        panic!("unreachable")
    }

    async fn handle(&self, _: Signal) {}

    async fn process(&self) {}

    async fn after_start(&self) {}
    
    fn before_stop(&self) {}

    fn get_schedule_mixin(&self) -> &ScheduleMixin;

    fn set_active(&self, active: bool) {
        let schedule_mixin = self.get_schedule_mixin();
        let is_alive = schedule_mixin.is_alive.lock().unwrap();
        is_alive.set(active);
    }

    fn is_active(&self) -> bool {
        let schedule_mixin = self.get_schedule_mixin();
        return schedule_mixin.is_alive.lock().unwrap().get();
    }
}

pub trait Flush {
    fn get_flush_sedner(&self) -> &Arc<channel::Sender<Message>>;
}

pub fn background(scheduler: Arc<dyn ScheduleMixer>) {
    let cloned_scheduler = Arc::clone(&scheduler);
    task::spawn(async move {
        cloned_scheduler.set_active(true);
        while cloned_scheduler.is_active() {
            cloned_scheduler.process().await;
        }
    });
}