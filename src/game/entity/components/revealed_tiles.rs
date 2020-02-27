use super::{HashMap, RevealedTiles};
use crate::sys::element::Map;

impl RevealedTiles {
    pub fn new() -> Self {
        Self {
            revealed: HashMap::new(),
        }
    }

    pub fn new_set(&mut self, map: &Map) {
        if !self.revealed.contains_key(&map.name) {
            self.revealed
                .insert(map.name.clone(), vec![false; map.width * map.height]);
        }
    }
}
