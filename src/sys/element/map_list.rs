use super::{HashMap, Map, MapList};
use crate::sys::utils::FindBy;

impl MapList {
    pub fn new() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }

    pub fn insert(&mut self, map: Map) -> &mut Map {
        self.maps.entry(map.name.clone()).or_insert(map)
    }

    pub fn find<'r, K: Into<FindBy<'r>>>(&self, name: K) -> &Map {
        match name.into() {
            FindBy::S(k) => &self.maps[k],
            FindBy::ST(k) => &self.maps[k],
            _ => panic!("expected String or &str, was usize"),
        }
    }

    pub fn find_mut<'r, K: Into<FindBy<'r>>>(&mut self, name: K) -> Option<&mut Map> {
        match name.into() {
            FindBy::S(k) => self.maps.get_mut(k),
            FindBy::ST(k) => self.maps.get_mut(k),
            _ => panic!("expected String or &str, was usize"),
        }
    }
}
