use Compass::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Compass {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Center,
}

const ROTATIONS_CW90: [Compass; 9] = [
    West, NorthWest, North, NorthEast, East, SouthEast, South, SouthWest, Center,
];

const ROTATIONS_CCW90: [Compass; 9] = [
    East, SouthEast, South, SouthWest, West, NorthWest, North, NorthEast, Center,
];

const ROTATIONS_180: [Compass; 9] = [
    South, SouthWest, West, NorthWest, North, NorthEast, East, SouthEast, Center,
];

const NORMALIZED_COORDS: [(i32, i32); 9] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 0),
];

impl Compass {
    pub fn rot_ccw90(self) -> Self {
        ROTATIONS_CCW90[self as usize]
    }

    pub fn rot_cw90(self) -> Self {
        ROTATIONS_CW90[self as usize]
    }

    pub fn rot_180(self) -> Self {
        ROTATIONS_180[self as usize]
    }

    pub fn normalized_coords(self) -> (i32, i32) {
        NORMALIZED_COORDS[self as usize]
    }
}
