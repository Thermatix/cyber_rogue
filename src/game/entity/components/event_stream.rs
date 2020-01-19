use super::{Component, EventStream, HashMap, VecStorage};
use std::convert::From;
use std::convert::Into;

type Events = Vec<Event>;
pub type Stream = HashMap<String, Events>;

#[derive(PartialEq, Debug)]
pub struct Event {
    pub message: String,
    pub value: EventValue,
}

#[derive(PartialEq, Debug)]
pub enum EventValue {
    Numeric(i32),
    Bool(bool),
    Text(String),
}

// let a: Option<String> = animal.into();
// let b: Option<bool> = animal.into();
// let b: Option<i32> = animal.into();
// a.unwrap()

impl From<i32> for EventValue {
    fn from(v: i32) -> Self {
        EventValue::Numeric(v)
    }
}

impl From<String> for EventValue {
    fn from(v: String) -> Self {
        EventValue::Text(v)
    }
}

impl From<bool> for EventValue {
    fn from(v: bool) -> Self {
        EventValue::Bool(v)
    }
}

impl Event {
    pub fn new<V: Into<EventValue>>(message: String, value: V) -> Event {
        Self {
            message: message,
            value: value.into(),
        }
    }
}

impl EventStream {
    pub fn new() -> Self {
        Self {
            stream: Stream::new(),
        }
    }

    pub fn add(&mut self, channel: &str, event: Event) {
        let chnl = channel.to_owned();
        if !self.stream.contains_key(&chnl) {
            self.stream.insert(chnl.clone(), Events::new());
        }

        self.stream.get_mut(&chnl).unwrap().push(event);
    }

    pub fn pop(&mut self, channel: &str) -> Option<&mut Events> {
        self.stream.get_mut(channel)
    }
}
