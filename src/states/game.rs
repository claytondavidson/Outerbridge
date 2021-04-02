use amethyst::{SimpleState, StateData, GameData};
//use amethyst::assets::{Loader, AssetStorage, ProgressCounter};
use amethyst::renderer::{SpriteRender, Camera};
use amethyst::core::ecs::{WorldExt, World, Builder, Component, DenseVecStorage};
//use crate::entities::Terrain;
//use amethyst::tiles::{TileMap, MortonEncoder};
use crate::resources::{SpriteResource, Sprite};
use amethyst::core::{Transform};
//use amethyst::core::math::{Vector3};
//use std::f32::consts::FRAC_PI_3;
use crate::systems::Player;
use crate::systems::PlayerTag;
use crate::systems::CameraTag;
//use rand::prelude::*;
use crate::resources::*;

#[derive(Component)]
struct Position {
    _x: i32,
    _y: i32,
}

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.insert(SpriteResource::new(&data.world));

        initialize_camera(data.world);
        create_player(data.world);
        draw_atlas(data.world);
    }
}

pub fn create_player(world: &mut World) {
    //let atlas = world.read_resource::<Atlas>();
    let sprite = world.read_resource::<SpriteResource>().get(Sprite::Player).unwrap();
    let sprite_render = SpriteRender::new(sprite, 0);
    let player_tag = PlayerTag::default();

    let mut transform = Transform::default();

   // let (x, y) = atlas.rooms[0].center();

    transform.set_translation_xyz(40.0, 25.0, 0.0);
    world
        .create_entity()
        .with(Player{})
        .with(sprite_render)
        .with(player_tag)
        .with(transform)
        .build();
}

/*pub fn create_terrain(world: &mut World) {
    let quantity;

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    {
        quantity = Terrain::QUANTITY.abs().ceil() as u32;
    }

    let tile_map = TileMap::<Terrain>::new(
        Vector3::new(20, 20, 1),
        Vector3::new(64, 64, 1),
        world.read_resource::<SpriteResource>().get(Sprite::Grass),
    );

    let mut transform = Transform::default();

    world
        .create_entity()
        .with(tile_map)
        .with(transform)
        .build();
}*/

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    let camera_tag = CameraTag::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(79.0, 49.0))
        .with(camera_tag)
        .with(transform)
        .build();
}