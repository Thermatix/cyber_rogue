use rltk::{Console, GameState, Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs::world::EntitiesRes;

// Game should not be here really
use crate::game::entity;
// use crate::game::entity::*;
use crate::game::system::*;
use crate::sys::element;

pub struct State {
    pub ecs: World,
}

impl State {
    pub fn new() -> Self {
        let mut s = Self { ecs: World::new() };
        entity::register_components(&mut s.ecs);
        s
    }

    // TODO: TRANSFORM TO USE a DISPATCHER
    // https://docs.rs/specs/0.15.1/specs/#system-execution
    fn run_systems(&mut self) {
        let mut lw = LeftWalker::new();
        let mut pw = Movement::new();
        lw.run_now(&self.ecs);
        pw.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn render_map(&mut self, map_name: &str, ctx: &mut Rltk) {
        let map_list = &self.ecs.fetch::<element::MapList>();
        let tile_list = &self.ecs.fetch::<element::TileSetList>();
        let renderables = &self.ecs.read_storage::<entity::Renderable>();
        let entity_store = &self.ecs.fetch::<EntitiesRes>();

        let map = map_list.find(map_name);
        let tile_set = tile_list.find(&map.tileset);
        let mut y = 0;
        let mut x = 0;

        for tn in &map.tiles {
            let tile = &tile_set.find(tn.t());
            if let Some(ent) = tn.entities.last() {
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

    // TODO: figure out way to uniquely identify player
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
                        println!("KeyPresed: {:?}", key);
                    }
                },
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();
        self.player_input(ctx);
        self.render_map("Test Map", ctx);
        // self.render_entities(ctx);
    }
}
