use maze;
use maze::Wall;

const AREA_FLAG_MASK_WALL_NORTH: u8 = 0b0000_0001;
const AREA_FLAG_MASK_WALL_SOUTH: u8 = 0b0000_0010;
const AREA_FLAG_MASK_WALL_WEST:  u8 = 0b0000_0100;
const AREA_FLAG_MASK_WALL_EAST:  u8 = 0b0000_1000;

/// An area.
pub struct Area {
    flags: u8,
    warmth: f32,
    humidity: f32,
}

impl maze::Cell for Area {
    fn set_wall(&mut self, wall: Wall) {
        let mask = match wall {
            Wall::North => AREA_FLAG_MASK_WALL_NORTH,
            Wall::South => AREA_FLAG_MASK_WALL_SOUTH,
            Wall::West  => AREA_FLAG_MASK_WALL_WEST,
            Wall::East  => AREA_FLAG_MASK_WALL_EAST,
        };
        self.flags |= mask;
    }
}
