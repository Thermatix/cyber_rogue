use crate::game::entity;
use rltk::{Console, GameState, Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::game::entity::*;
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
    // TODO: dynamically load and execute system
    fn run_systems(&mut self) {
        let mut lw = LeftWalker::new();
        let mut pw = PlayerMovement::new();
        lw.run_now(&self.ecs);
        pw.run_now(&self.ecs);
        self.ecs.maintain();
    }

    // TODO: move to own system
    fn render_entities(&mut self, ctx: &mut Rltk) {
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, *render.g());
        }
    }

    fn render_map(&mut self, map_name: &str, ctx: &mut Rltk) {
        let map_list = &self.ecs.fetch::<element::MapList>();
        let map = map_list.find(map_name);
        let tile_list = &self.ecs.fetch::<element::TileSetList>();
        let tile_set = tile_list.find(&map.tileset);
        let mut y = 0;
        let mut x = 0;

        for tn in &map.tiles {
            let tile = &tile_set.find(tn);
            ctx.set(x, y, tile.visual.fg, tile.visual.fg, *tile.visual.g());
            x += 1;
            if x > map.x as i32 {
                y += 1;
                x = 0;
            }
        }
    }

    // TODO: figure out way to uniquely identify player
    fn player_input(&mut self, ctx: &mut Rltk) {
        use player::Movements::*;

        let mut players = self.ecs.write_storage::<Player>();
        for player in (&mut players).join() {
            match ctx.key {
                None => {}
                Some(key) => match key {
                    VirtualKeyCode::Up => player.movements.push(Up(1)),
                    VirtualKeyCode::Left => player.movements.push(Left(1)),
                    VirtualKeyCode::Down => player.movements.push(Down(1)),
                    VirtualKeyCode::Right => player.movements.push(Right(1)),
                    _ => {}
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
        self.render_entities(ctx);
    }
}
