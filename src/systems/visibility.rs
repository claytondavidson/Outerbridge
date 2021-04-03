/*use amethyst::shred::System;
use amethyst::core::ecs::shred::DynamicSystemData;
use amethyst::core::ecs::{WriteStorage, Join, Entities, ReadStorage};
use crate::entities::View;
use crate::states::Position;
use crate::resources::{Atlas, Point};
use crate::systems::Player;
use std::collections::HashSet;
use crate::utils::{Algorithm2D, BresenhamCircle};

pub struct VisibilitySystem;

pub fn field_of_view_set(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> HashSet<Point> {
    let mut visible_points: HashSet<Point> = HashSet::with_capacity(((range * 2) * (range * 2)) as usize);

    BresenhamCircle::new(start.x, start.y, range).for_each(|point| {
        scan_fov_line(start, point, fov_check, &mut visible_points);
    });

    visible_points
}

pub fn field_of_view(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> Vec<Point> {
    field_of_view_set(start, range, fov_check).into_iter().collect()
}

fn scan_fov_line(
    start: Point,
    end: Point,
    fov_check: &dyn Algorithm2D,
    visible_points: &mut HashSet<Point>,
) {
    let line = VectorLine::new(start, end);

    for target in line {
        if !fov_check.in_bounds(target) {
            break;
        }
        visible_points.insert(target);
        if fov_check.is_opaque(fov_check.point2d_to_index(target)) {
            break;
        }
    }
}

impl<'s> System<'s> for VisibilitySystem {
    type SystemData = (
        ReadStorage<'s, Atlas>,
        Entities<'s>,
        WriteStorage<'s, View>,
        WriteStorage<'s, Position>,
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, (mut atlas, entities, mut view, pos, player) : Self::SystemData) {
        for (entity, view, pos) in (&entities, &mut view, &pos).join() {
            view.visible_tiles.clear();
            view.visible_tiles = field_of_view(Point::new(pos.x, pos.y), view.range, &*atlas);
            view.visible_tiles.retain(|p| p.x >= 0 && p.x < atlas.width && p.y >= 0 && p.y < atlas.height );

            // If this is the player, reveal what they can see
            let p : Option<&Player> = player.get(entity);
            if let Some(p) = p {
                for vis in view.visible_tiles.iter() {
                    let idx = atlas.xy_idx(vis.x, vis.y);
                    atlas.revealed_tiles[idx] = true;
                }
            }
        }
    }
}*/