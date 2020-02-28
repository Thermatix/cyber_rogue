use super::super::*;
use super::Visibility;
use crate::game::entity::{FieldOfView, Location, Player, Position, RevealedTiles};

use specs::prelude::*;

impl<'a> System<'a> for Visibility {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, crate::sys::element::MapList>,
        ReadStorage<'a, Location>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, RevealedTiles>,
        WriteStorage<'a, Position>,
    );

    fn run(
        &mut self,
        (entities, map_list, locations, player, mut fov, mut revealed_tiles, pos): Self::SystemData,
    ) {
        use rltk::{field_of_view, Point};
        for (ent, loc, fov, revealed_tiles, pos) in
            (&entities, &locations, &mut fov, &mut revealed_tiles, &pos).join()
        {
            if fov.dirty {
                let map = map_list.find(loc.current());

                revealed_tiles.new_set(&map);
                let rts = revealed_tiles.revealed.get_mut(&map.name).unwrap();
                let vts = revealed_tiles.visible.get_mut(&map.name).unwrap();

                fov.visible_tiles.clear();
                fov.visible_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
                fov.visible_tiles
                    .retain(|p| p.x > 0 && p.x < (map.x as i32) && p.y > 0 && p.y < (map.y as i32));

                let p: Option<&Player> = player.get(ent);
                if let Some(_) = p {
                    for t in vts.iter_mut() {
                        *t = false
                    }
                    for vis in &fov.visible_tiles {
                        let idx = map.xy_idx(vis.x as usize, vis.y as usize);
                        rts[idx] = true;
                        vts[idx] = true;
                    }
                }
                fov.dirty = false;
            }
        }
    }
}
