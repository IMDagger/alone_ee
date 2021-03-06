use crate::listener::{EventHandler, EventHandlerResult, Listener};
use crate::observer::Observer;
use crate::subscription::Subscription;
use odds::vec::VecExt;
use std::fmt;

#[derive(Debug, Default)]
pub struct EventEmitter<TEvent> {
    listeners: Vec<Listener<TEvent>>,
}

impl<TEvent> fmt::Display for EventEmitter<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EventEmitter<{}>", self.len())
    }
}

impl<TEvent> EventEmitter<TEvent> {
    #[inline(always)]
    pub fn new() -> Self {
        Self { listeners: Vec::new() }
    }

    #[inline(always)]
    pub fn on(&mut self, handler: EventHandler<TEvent>) -> Subscription {
        self.subscribe(Listener::new(false, handler))
    }

    #[inline(always)]
    pub fn once(&mut self, handler: EventHandler<TEvent>) -> Subscription {
        self.subscribe(Listener::new(true, handler))
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.listeners.is_empty()
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.listeners = Vec::new();
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.listeners.iter().filter(|l| l.is_active()).count()
    }

    #[inline(always)]
    pub fn emit(&mut self, message: &TEvent) -> EventHandlerResult {
        self.publish(message)
    }
}

impl<TEvent> Observer<TEvent> for EventEmitter<TEvent> {
    fn subscribe(&mut self, listener: Listener<TEvent>) -> Subscription {
        let subsc = Subscription::new(listener.get_activation_flag());
        self.listeners.push(listener);
        subsc
    }

    fn publish(&mut self, message: &TEvent) -> EventHandlerResult {
        let mut res = Ok(());

        self.listeners.retain_mut(|lst| {
            if res.is_err() {
                return true;
            }

            if !lst.is_active() {
                return false;
            }

            res = lst.call(message);

            !lst.once
        });

        res
    }
}

impl<TEvent> Drop for EventEmitter<TEvent> {
    fn drop(&mut self) {
        self.reset()
    }
}
