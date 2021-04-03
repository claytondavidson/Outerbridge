use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler},
};
use amethyst::core::ecs::NullStorage;
use crate::systems::{MovementBindingTypes, AxisBinding};
use crate::resources::{Atlas};
use crate::entities::TileType;

pub struct Player {
    //pub(crate) id: usize,
}

impl Player {
/*    pub fn shoot(&self) {
        println!("PEW! {}", self.id);
    }
*/}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}



#[derive(Default)]
pub struct PlayerTag;

impl Component for PlayerTag {
    type Storage = NullStorage<Self>;
}

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, Atlas>,
    );

    fn run(&mut self, (mut transform, player, input, atlas): Self::SystemData) {
        for (_player, transform) in (&player, &mut transform).join() {
            let horizontal = input
                .axis_value(&AxisBinding::Horizontal)
                .unwrap_or(0.0);
            let vertical = input
                .axis_value(&AxisBinding::Vertical)
                .unwrap_or(0.0);

            let mut player_x = 0.0;
            let mut player_y = 0.0;

            let trans_x = if vertical == 1. {
                0.
            } else {
                1.
            };

            let trans_y = if horizontal == 1. {
                0.
            } else {
                1.
            };

            player_x = transform.translation().x + trans_x;
            player_y = transform.translation().y + trans_y;

            let destination_idx = atlas.xy_idx((player_x + vertical).floor() as i32, (player_y + horizontal).floor() as i32);
            //println!("{} {} {}", horizontal, vertical, destination_idx);
            println!("{} {}", player_x, player_y);
            println!("index: {}", destination_idx);

            /*let mut counter = 0;
            for tile in &atlas.tiles {
                print!("type: {:?} idx: {} ", tile, counter);
                counter += 1;
            }*/

            if atlas.tiles[destination_idx] != TileType::Grass {
                transform.move_up(horizontal / 8.);
                transform.move_right(vertical / 8.);
            }

/*            let shoot = input
                .action_is_down(&ActionBinding::Shoot)
                .unwrap_or(false);

            if shoot {
                player.shoot();
            }
*/        }
    }
}