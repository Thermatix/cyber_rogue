use super::{HashMap, Map, MapList};

impl<'ml> MapList<'ml> {
    pub fn new() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }

    pub fn insert(&mut self, map: Map<'ml>) {
        self.maps.insert(map.name, map);
    }

    pub fn find(&self, name: &'ml str) -> &Map {
        &self.maps[name]
    }
}
