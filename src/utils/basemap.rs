use crate::resources::Point;

pub trait BaseMap {
    fn is_opaque(&self, idx: i32) -> bool;

    fn get_available_exits(&self, idx: i32) -> Vec<(i32, f32)>;

    fn get_pathing_distance(&self, idx1: i32, idx2: i32) -> f32;
}

pub trait Algorithm2D: BaseMap {
    fn point2d_to_index(&self, pt: Point) -> i32;

    fn index_to_point2d(&self, idx: i32) -> Point;

    fn in_bounds(&self, _pos : Point) -> bool { true }
}