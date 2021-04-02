use amethyst::core::ecs::{Component, DenseVecStorage, World, WorldExt};
use amethyst::core::Transform;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use crate::resources::{SpriteResource, Sprite};
use amethyst::assets::Handle;

pub struct Tile {
    pub tile_type: TileType,
    pub blocking: bool,
    pub hit_box: [[f32; 2]; 2],
    pub center: [f32; 2],
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Grass,
            blocking: false,
            hit_box: [[0.0, 0.0], [0.0, 0.0]],
            center: [0.0, 0.0],
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Grass,
}

pub fn initialize_tile(world: &mut World, tile_type: TileType, sprite_sheet_handle: Handle<SpriteSheet>, center: [f32; 2]) -> Tile {
    let mut transform = Transform::default();
    transform.set_translation_xyz(center[0], center[1], 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let blocking = match tile_type {
        TileType::Wall => false,
        _ => true,
    };

    let lower_x = center[0] - 32.0 / 2.0;
    let lower_y = center[1] - 32.0 / 2.0;
    let upper_x = center[0] + 32.0 / 2.0;
    let upper_y = center[1] + 32.0 / 2.0;
    let hit_box = [[lower_x, lower_y], [upper_x, upper_y]];

    Tile { tile_type, blocking, hit_box, center }
}