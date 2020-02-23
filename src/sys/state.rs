use rltk::{Console, GameState, Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs::world::{EntitiesRes, EntityBuilder};
use specs::{Dispatcher, Entity, WorldExt};

// Game should not be here really
use crate::game::entity;
// use crate::game::entity::*;
use crate::game::system::*;
use crate::sys::element;

pub struct State {
    pub ecs: World,
    pub dsp: Dispatcher<'static, 'static>,
}

impl State {
    pub fn new(
        build_dispatcher: &dyn Fn() -> Dispatcher<'static, 'static>,
        register_components: &dyn Fn(World) -> World,
    ) -> Self {
        Self {
            ecs: register_components(World::new()),
            dsp: build_dispatcher(),
        }
    }

    fn render_map(&mut self, map_name: &str, ctx: &mut Rltk) {
        let map_list = &self.ecs.fetch::<element::MapList>();
        let renderables = &self.ecs.read_storage::<entity::Renderable>();
        let entity_store = &self.ecs.fetch::<EntitiesRes>();

        // Store and use handler function that returns the map and tile_set
        let map = map_list.find(map_name);
        let mut y = 0;
        let mut x = 0;

        for tn in &map.tiles {
            let tile = &map.tile_set.find(tn);
            if let Some(ent) = map.entities[&(x as usize)][&(y as usize)].last() {
                let r = renderables.get(entity_store.entity(*ent)).unwrap();
                ctx.set(x, y, r.fg, tile.visual.bg, *r.g());
            } else {
                ctx.set(x, y, tile.visual.fg, tile.visual.bg, *tile.visual.g());
            }
            x += 1;
            if x > map.x as i32 {
                y += 1;
                x = 0;
            }
        }
    }

    pub fn insert_map<F>(&mut self, tile_set: &str, mut block: F)
    where
        F: FnMut(element::TileSet) -> element::Map,
    {
        let map_list = &mut self.ecs.fetch_mut::<element::MapList>();
        let tile_sets = &self.ecs.fetch::<element::TileSetList>();
        map_list.insert(block(tile_sets.find(tile_set).clone()));
    }

    pub fn modify_map<'c, 'm: 'c, F>(&'m mut self, map_name: &'c str, mut block: F)
    where
        F: FnMut(&'m mut element::Map) + 'c,
    {
        let mut map_list = self.ecs.fetch_mut::<element::MapList>();
        match map_list.find_mut(map_name) {
            Some(mut map) => block(&mut map),
            None => (),
        };
    }

    pub fn insert_entity<F>(&mut self, mut block: F) -> u32
    where
        F: FnMut(EntityBuilder) -> EntityBuilder,
    {
        block(self.ecs.create_entity()).build().id()
    }

    fn player_input(&mut self, ctx: &mut Rltk) {
        let mut players = self.ecs.write_storage::<entity::Player>();
        let mut events = self.ecs.write_storage::<entity::EventStream>();
        for (_player, event) in (&mut players, &mut events).join() {
            match ctx.key {
                None => {}
                Some(key) => match key {
                    VirtualKeyCode::Up => {
                        println!("Pressed: {}", "up");
                        event.add_to_channel("motions", ("y", -1))
                    }
                    VirtualKeyCode::Left => {
                        println!("Pressed: {}", "left");
                        event.add_to_channel("motions", ("x", -1))
                    }
                    VirtualKeyCode::Down => {
                        println!("Pressed: {}", "down");
                        event.add_to_channel("motions", ("y", 1))
                    }
                    VirtualKeyCode::Right => {
                        println!("Pressed: {}", "right");
                        event.add_to_channel("motions", ("x", 1))
                    }
                    _ => {
                        println!("KeyPressed: {:?}", key);
                    }
                },
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.player_input(ctx);
        self.dsp.dispatch(&mut self.ecs);
        self.ecs.maintain();
        self.render_map("Test Map", ctx);
    }
}
