use super::{EventStream, HashMap};
use std::convert::Into;

type Events = Vec<Event>;
pub type Stream = HashMap<String, Events>;

#[derive(PartialEq, Debug)]
struct Event {
    pub msg: String,
    pub value: EventValue,
}

#[derive(PartialEq, Debug)]
enum EventValue {
    Numeric(i32),
    Text(String),
}

// let a: Option<String> = animal.into();
// a.unwrap()
impl Into<Option<String>> for EventValue {
    fn into(self) -> Option<String> {
        match self {
            EventValue::Text(u) => Some(u),
            _ => None,
        }
    }
}

// let a: Option<i32> = animal.into();
// a.unwrap()
impl Into<Option<i32>> for EventValue {
    fn into(self) -> Option<i32> {
        match self {
            EventValue::Numeric(u) => Some(u),
            _ => None,
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
            self.stream.insert(chnl, Events::new());
        }

        self.stream[&chnl].push(event);
    }

    pub fn pop(&mut self, channel: &str) -> Option<Event> {
        self.stream[channel].pop()
    }
}
