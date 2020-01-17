use super::Location;

impl Location {
    pub fn new() -> Self {
        Self {
            pointer: Vec::new(),
        }
    }

    pub fn travel(&mut self, loc: String) {
        self.pointer.push(loc);
    }

    pub fn leave(&mut self) -> Option<String> {
        self.pointer.pop()
    }
}
