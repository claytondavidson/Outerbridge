use amethyst::core::ecs::{World, WorldExt, Builder};
use crate::resources::{SpriteResource, Sprite};
use amethyst::renderer::SpriteRender;
use amethyst::core::Transform;
use rand::{Rng, thread_rng};
use std::cmp::{max, min};
use crate::entities::TileType;
use amethyst::assets::AssetStorage;
use crate::utils::Algorithm2D;

#[derive(Clone)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x:i32, y: i32, w:i32, h:i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn intersect(&self, other_rect: &Rect) -> bool {
        self.x1 <= other_rect.x2 && self.x2 >= other_rect.x1 && self.y1 <= other_rect.y2 && self.y2 >= other_rect.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ( (self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2 )
    }
}

#[derive(Default, Clone)]
pub struct Atlas {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Atlas {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn new_room(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Wall;
            }
        }
    }

    fn new_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        for x in min(x1,x2) ..= max(x1,x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Wall;
            }
        }
    }

    fn new_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1,y2) ..= max(y1,y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Wall;
            }
        }
    }

    pub fn new_atlas_with_rooms_and_stuff() -> Self {
        let mut atlas = Self {
            tiles: vec![TileType::Grass; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = thread_rng();

        for _ in 0..MAX_ROOMS {
            let width = rng.gen_range(MIN_SIZE..MAX_SIZE);
            let height = rng.gen_range(MIN_SIZE..MAX_SIZE);
            let x = roll_dice(1, atlas.width - width - 1) - 1;
            let y = roll_dice(1, atlas.height - height - 1) - 1;
            let new_room = Rect::new(x, y, width, height);
            let mut flag = true;
            for other_room in atlas.rooms.iter() {
                if new_room.intersect(other_room) {
                    flag = false
                }
            }
            if flag {
                atlas.new_room(&new_room);

                if !atlas.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = atlas.rooms[atlas.rooms.len() - 1].center();
                    if rng.gen_range(0..2) == 1 {
                        atlas.new_horizontal_tunnel(prev_x, new_x, prev_y);
                        atlas.new_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        atlas.new_vertical_tunnel(prev_y, new_y, prev_x);
                        atlas.new_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }
                atlas.rooms.push(new_room);
            }
        }
        atlas
    }
}

/*pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Grass; 80*50];

    // Make the boundaries walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = rand::thread_rng();

    for _i in 0..400 {
        let x = rng.gen_range(1..=79);
        let y = rng.gen_range(1..=49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}
*/

fn roll_dice(n: i32, die_type: i32) -> i32 {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen_range(1..die_type + 1)).sum()
}

pub fn draw_atlas(world: &mut World) {

    let tiles = {
        let atlas = world.read_resource::<Atlas>();
        atlas.tiles.clone()
    };

    let mut counter = 0;
    for tile in &tiles {
        print!("type: {:?} idx: {} ", tile, counter);
        counter += 1;
    }
    //println!("{:?}", atlas.tiles);

    let mut y = 0.;
    let mut x = 0.;

    let mut counter = 0;

    for (idx, tile) in tiles.iter().enumerate() {
        let grass = world.read_resource::<SpriteResource>().get(Sprite::Grass).unwrap();
        let wall = world.read_resource::<SpriteResource>().get(Sprite::Wall).unwrap();
        let grass_render = SpriteRender::new(grass, 0);
        let wall_render = SpriteRender::new(wall, 0);
        let mut transform = Transform::default();

        transform.set_translation_xyz(x, y, 0.0);
        match tile {
            TileType::Grass => {
                world
                    .create_entity()
                    .with(grass_render)
                    .with(transform)
                    .build();
                }
            TileType::Wall => {
                world
                    .create_entity()
                    .with(wall_render)
                    .with(transform)
                    .build();
            }
        }

        counter += 1;

        x += 1.;
        if x > 79. {
            x = 0.;
            y += 1.;
        }
    }
    println!("{}", counter);
    println!("done");
}

pub struct Point {
    width: i32,
    height: i32,
}

impl Point {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}