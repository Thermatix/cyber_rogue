use super::Event;

#[derive(PartialEq, Debug)]
pub enum EventValue {
    Numeric(i32),
    Bool(bool),
    Text(String),
    Point((i32, i32)),
}

impl From<(i32, i32)> for EventValue {
    fn from(v: (i32, i32)) -> Self {
        EventValue::Point(v)
    }
}

impl From<EventValue> for (i32, i32) {
    fn from(v: EventValue) -> Self {
        match v {
            EventValue::Point(n) => n,
            _ => panic!("{:?} was not a Point (i32,i32)", v),
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

impl From<(&str, (i32, i32))> for Event {
    fn from((msg, v): (&str, (i32, i32))) -> Self {
        Event::new(msg.to_owned(), v)
    }
}

impl From<(String, (i32, i32))> for Event {
    fn from((msg, v): (String, (i32, i32))) -> Self {
        Event::new(msg, v)
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
