use super::super::*;
use super::Visibility;
use crate::game::entity::{FieldOfView, Location, Position};

use specs::prelude::*;

impl<'a> System<'a> for Visibility {
    type SystemData = (
        ReadExpect<'a, crate::sys::element::MapList>,
        ReadStorage<'a, Location>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (map_list, locations, mut fov, pos): Self::SystemData) {
        use rltk::{field_of_view, Point};
        for (loc, fov, pos) in (&locations, &mut fov, &pos).join() {
            let map = map_list.find(loc.current());
            fov.visible_tiles.clear();
            fov.visible_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
            fov.visible_tiles.retain(|p| {
                p.x > 0 && p.x < map.width as i32 - 1 && p.y > 0 && p.y < map.height as i32 - 1
            });
        }
    }
}
