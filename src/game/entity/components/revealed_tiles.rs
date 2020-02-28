use super::{HashMap, RevealedTiles};
use crate::sys::element::Map;

impl RevealedTiles {
    pub fn new() -> Self {
        Self {
            revealed: HashMap::new(),
            visible: HashMap::new(),
        }
    }

    pub fn new_set(&mut self, map: &Map) {
        if !self.revealed.contains_key(&map.name) {
            let vec = vec![false; map.width * map.height];
            self.revealed.insert(map.name.clone(), vec.clone());
            self.visible.insert(map.name.clone(), vec);
        }
    }
}
