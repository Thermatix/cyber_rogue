use super::{Component, EventStream, EventValue, HashMap, VecStorage};
use std::convert::{From, Into};

#[derive(PartialEq, Debug)]
pub struct Event {
    pub message: String,
    pub value: EventValue,
}

type Events = Vec<Event>;
pub type Stream = HashMap<String, Events>;

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
