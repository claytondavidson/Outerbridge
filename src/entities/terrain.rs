use amethyst::{
    core::math::Point3,
    ecs::{World},
    tiles::{Tile},
};

#[derive(Clone, Default)]
pub struct Terrain;

impl Terrain {
    //pub const SIZE: u32 = 4;
    //pub const SIZE_HALF: f32 = Self::SIZE as f32 / 2.0;
    //pub const QUANTITY: f32 = 15.0 / Self::SIZE as f32 + 0.5;
}

impl Tile for Terrain {
    fn sprite(&self, _coords: Point3<u32>, _: &World) -> Option<usize> {
        Some(0)
    }
}
