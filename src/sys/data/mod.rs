use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use crate::game::entity;

mod conditions;
mod spawn_group;
pub type Components = Vec<entity::Component>;
pub type EntityGroup = HashMap<String, Components>;
pub type EntityKinds = HashMap<String, EntityGroup>;

pub struct SpawnGroup {
    pub entities: Vec<Spawn>,
    pub conditions: spawn_group::ConditionsToSpawn,
}

pub struct Spawn {
    pub name: String,
    pub kind: String,
    pub to_spawn: i32,
    pub conditions: spawn_group::ConditionsToSpawn,
}

pub struct SpawnTable {
    pub entities: EntityKinds,
}

impl SpawnTable {
    pub fn new() -> Self {
        Self {
            entities: EntityKinds::new(),
        }
    }

    pub fn load(file_path: String) -> Self {
        let mut st = Self::new();
        Self::get_data(st)
    }

    pub fn get_data(mut st: Self) -> Self {
        st["mobs"] = EntityGroup::new();
        st["items"] = EntityGroup::new();
        st["mobs"]["goblin"] = SpawnTable::std_components(vec![]);
        st
    }

    pub fn std_components(mut comps: Components) -> Components {
        use entity::*;

        comps.push(Component::Position(Position::new(0, 0)));
        comps.push(Component::Location(Location::new("")));
        comps
    }
}

impl Index<&str> for SpawnTable {
    type Output = EntityGroup;

    fn index(&self, index: &str) -> &Self::Output {
        &self.entities[index]
    }
}

impl IndexMut<&str> for SpawnTable {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.entities.get_mut(index).unwrap()
    }
}
