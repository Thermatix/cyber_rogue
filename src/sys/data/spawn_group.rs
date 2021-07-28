use super::{Spawn, SpawnGroup};
use specs::prelude::World;

pub type ConditionsToSpawn = Vec<Conditions>;
pub type EntityIdent = String;
pub type ToSpawn = (EntityIdent, i32);
pub type SpawnList = Vec<ToSpawn>;

#[derive(Debug, PartialEq)]
pub enum Conditions {
    None,
}

impl Spawn {
    pub fn to_spawn(&self, ecs: &World) -> Option<ToSpawn> {
        if self.conditions.iter().all(|c| c.passed(&ecs)) {
            Some((self.name.clone(), self.to_spawn))
        } else {
            None
        }
    }
}

impl SpawnGroup {
    pub fn to_spawn(&self, ecs: &World) -> SpawnList {
        let mut spawn_list = SpawnList::new();
        if self.conditions.iter().all(|c| c.passed(&ecs)) {
            self.entities.iter().for_each(|s| match s.to_spawn(&ecs) {
                Some(spawnable) => spawn_list.push(spawnable),
                _ => {}
            })
        }
        spawn_list
    }
}
