use crate::game::entity;
use rltk::{Console, GameState, Rltk};
use specs::prelude::*;

use crate::game::entity::*;
use crate::game::system::*;

pub struct State {
    pub ecs: World,
}

impl State {
    pub fn new() -> Self {
        let mut s = Self { ecs: World::new() };
        entity::register_components(&mut s.ecs);
        s
    }

    fn run_systems(&mut self) {
        let mut lw = LeftWalker::new();
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }

    // TODO: move to own system
    fn render_entities(&mut self, ctx: &mut Rltk) {
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();

        self.render_entities(ctx);
    }
}
