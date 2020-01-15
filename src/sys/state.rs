use crate::game::entity;
use rltk::{Console, GameState, Rltk};
use specs::prelude::*;

use crate::game::entity::components::{Position, Renderable};

pub struct State {
    pub ecs: World,
}

impl State {
    pub fn new() -> Self {
        let mut s = Self { ecs: World::new() };
        entity::register_components(&mut s.ecs);
        s
    }

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
        self.render_entities(ctx);
    }
}
