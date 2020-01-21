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

impl From<i32> for EventValue {
    fn from(v: i32) -> Self {
        EventValue::Numeric(v)
    }
}

impl From<EventValue> for i32 {
    fn from(v: EventValue) -> Self {
        match v {
            EventValue::Numeric(n) => n,
            _ => panic!("{:?} was not Numeric", v),
        }
    }
}

impl From<String> for EventValue {
    fn from(v: String) -> Self {
        EventValue::Text(v)
    }
}

impl From<EventValue> for String {
    fn from(v: EventValue) -> Self {
        match v {
            EventValue::Text(s) => s,
            _ => panic!("{:?} was not String", v),
        }
    }
}

impl From<bool> for EventValue {
    fn from(v: bool) -> Self {
        EventValue::Bool(v)
    }
}

impl From<EventValue> for bool {
    fn from(v: EventValue) -> Self {
        match v {
            EventValue::Bool(b) => b,
            _ => panic!("{:?} was not Bool", v),
        }
    }
}
impl From<(&str, i32)> for Event {
    fn from((msg, v): (&str, i32)) -> Self {
        Event::new(msg.to_owned(), v)
    }
}

impl From<(&str, String)> for Event {
    fn from((msg, v): (&str, String)) -> Self {
        Event::new(msg.to_owned(), v)
    }
}

impl From<(String, &str)> for Event {
    fn from((msg, v): (String, &str)) -> Self {
        Event::new(msg, v.to_owned())
    }
}

impl From<(&str, &str)> for Event {
    fn from((msg, v): (&str, &str)) -> Self {
        Event::new(msg.to_owned(), v.to_owned())
    }
}

impl From<(&str, bool)> for Event {
    fn from((msg, v): (&str, bool)) -> Self {
        Event::new(msg.to_owned(), v)
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

    pub fn add_to_channel<V: Into<Event>>(&mut self, channel: &str, event: V) {
        let chnl = channel.to_owned();
        if !self.stream.contains_key(&chnl) {
            self.stream.insert(chnl.clone(), Events::new());
        }

        self.stream.get_mut(&chnl).unwrap().push(event.into());
    }

    pub fn get_channel(&mut self, channel: &str) -> Option<&mut Events> {
        self.stream.get_mut(channel)
    }
}
