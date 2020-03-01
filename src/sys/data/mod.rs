use std::collections::HashMap;

mod conditions;
mod spawn_group;

pub struct SpawnGroup {
    pub entities: Vec<spawn_group::Spawn>,
    pub conditions: spawn_group::ConditionsToSpawn,
}
