//! Stores the location for given entity
use super::Location;

impl Location {
    pub fn new(initial: &str) -> Self {
        let mut l = Self { stack: Vec::new() };
        l.stack.push(initial.to_owned());
        l
    }

    pub fn travel(&mut self, loc: String) {
        self.stack.push(loc);
    }

    pub fn leave(&mut self) -> Option<String> {
        self.stack.pop()
    }

    pub fn current(&self) -> &String {
        self.stack.last().unwrap()
    }
}
