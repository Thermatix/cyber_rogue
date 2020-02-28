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

    fn render_map(&mut self, ctx: &mut Rltk) {
        let map_list = &self.ecs.fetch::<element::MapList>();
        let renderables = &self.ecs.read_storage::<entity::Renderable>();
        let locations = &self.ecs.read_storage::<entity::Location>();
        let revealed_tiles = &self.ecs.read_storage::<entity::RevealedTiles>();
        let player = &self.ecs.read_storage::<entity::Player>();
        let entity_store = &self.ecs.fetch::<EntitiesRes>();

        // Store and use handler function that returns the map and tile_set
        let mut y = 0;
        let mut x = 0;

        for (_, loc, tiles) in (player, locations, revealed_tiles).join() {
            let map = map_list.find(loc.current());
            let rts = &tiles.revealed[&map.name];
            let vts = &tiles.visible[&map.name];
            for (idx, tn) in map.tiles.iter().enumerate() {
                if rts[idx] {
                    let tile = &map.tile_set.find(tn);
                    let (mut fg, mut bg, g) = match map.entities.get(&(x as usize)) {
                        Some(row) => match row.get(&(y as usize)) {
                            Some(entities) => match entities.last() {
                                Some(ent) => {
                                    let r = renderables.get(entity_store.entity(*ent)).unwrap();
                                    (r.fg, tile.visual.bg, r.g())
                                }
                                None => (tile.visual.fg, tile.visual.bg, tile.visual.g()),
                            },
                            None => (tile.visual.fg, tile.visual.bg, tile.visual.g()),
                        },
                        None => (tile.visual.fg, tile.visual.bg, tile.visual.g()),
                    };

                    if !vts[idx] {
                        fg = rltk::RGB::named(rltk::GRAY10);
                        bg = rltk::RGB::named(rltk::GRAY0);
                    };
                    ctx.set(x, y, fg, bg, *g);
                }

                x += 1;
                if x > map.x as i32 {
                    y += 1;
                    x = 0;
                }
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

    // pub fn modify_map<'c, 'm: 'c, F>(&'m mut self, map_name: &'c str, mut block: F)
    // where
    //     F: FnMut(&'m mut element::Map) + 'c,
    // {
    //     let mut map_list = self.ecs.fetch_mut::<element::MapList>();
    //     match map_list.find_mut(map_name) {
    //         Some(mut map) => block(&mut map),
    //         None => (),
    //     };
    // }

    pub fn insert_entity<F>(&mut self, mut block: F) -> u32
    where
        F: FnMut(EntityBuilder) -> EntityBuilder,
    {
        block(self.ecs.create_entity()).build().id()
    }

    fn player_input(&mut self, ctx: &mut Rltk) {
        let mut events = self.ecs.fetch_mut::<entity::EventStream>();
        // THIS SHOULD OUTPUT TO INPUT CHANNEL AND LET SYSTEMS LOOKING
        // FOR CERTAIN INPUTS PULL THEM OFF AND DO SOMETHING WITH THEM
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Up => {
                    println!("Pressed: {}", "up");
                    events.add_to_channel("key_press", ("u", 0))
                }
                VirtualKeyCode::Left => {
                    println!("Pressed: {}", "left");
                    events.add_to_channel("key_press", ("l", 1))
                }
                VirtualKeyCode::Down => {
                    println!("Pressed: {}", "down");
                    events.add_to_channel("key_press", ("d", 2))
                }
                VirtualKeyCode::Right => {
                    println!("Pressed: {}", "right");
                    events.add_to_channel("key_press", ("r", 3))
                }
                _ => {
                    println!("KeyPressed: {:?}", key);
                }
            },
        };
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.player_input(ctx);
        self.dsp.dispatch(&mut self.ecs);
        self.ecs.maintain();
        self.render_map(ctx);
    }
}
