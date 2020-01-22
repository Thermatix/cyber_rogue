use super::{HashMap, Map, MapList};

impl MapList {
    pub fn new() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }

    pub fn insert(&mut self, map: Map) {
        self.maps.insert(map.name.clone(), map);
    }

    pub fn find(&self, name: &str) -> &Map {
        &self.maps[name]
    }
    pub fn find_mut(&mut self, name: &str) -> Option<&mut Map> {
        self.maps.get_mut(name)
    }
}
