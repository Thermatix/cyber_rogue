# Todo
## Impliment spawning(feature/entity_spawning):

* Entity Spawn Process
  - Iterate over map sectors(rooms)
  - For each sector generate a list of viable positions(within sector boundries, not on blocked tile)
  - Pass information to the EntityBuilder [player location, list of positions, tags(castle, underground, weak)]
  - Entity Builder find SpawnGroup with matching number of entities & tags
  - for each item in the returned spawn list (SpawnGroup.to_spawn -> [entity_kind, entity name, number to spawn])
    find the corrosponding raw data in the spawnTable and then build the entity, set it's position by poping one
    off the list of given positions & clone the given location

* [ ] Entity Spawn groups
  - [ ] One spawn group per sector/room
  - [ ] Spawn groups can be random from pool or specific or both
  - [x] Spawns groups and spawns should be blockable based on inputable values(level, item, switch, etc)
  - [ ] Spawns should be able to spawn based on configurable inputable values (level, item, switch, etc)
* [ ] Spawn Table
  - [ ] Spawntable stores raw entity data
  - [ ] raw entities are version of the components that are then cloned when building the entity
  - [ ] Spawn Table will return the parts of the given entity and nothing else
* [ ] Entity builder
  - Actually builds the requisit entities
  - Will first request a spawn group from Entity spawn groups
  - Will then request entity data for each entity in the spawn group from the Spawn table
  - will then build the given entities by itterating over the list of raw entity components and passing them to Specs::EntityBuilder::build
  
