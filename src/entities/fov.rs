use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct View {
    pub visible_tiles: Vec<(f32, f32)>,
    pub range: i32,
}

impl Component for View {
    type Storage = DenseVecStorage<Self>;
}

impl View {
    pub(crate) fn new() -> Self {
        Self {
            visible_tiles: Vec::new(),
            range: 8,
        }
    }
}