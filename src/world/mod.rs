use color;
use direction::Direction;
use log::Log;

const AREA_FLAG_MASK_WALL_NORTH: u8 = 0b0000_0001;
const AREA_FLAG_MASK_WALL_SOUTH: u8 = 0b0000_0010;
const AREA_FLAG_MASK_WALL_WEST:  u8 = 0b0000_0100;
const AREA_FLAG_MASK_WALL_EAST:  u8 = 0b0000_1000;

/// An area.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Area {
    flags: u8,
    warmth: f32,
    humidity: f32,
}

impl Area {
    pub fn new() -> Self {
        Area{
            flags: 0b0000_0000,
            warmth: 0.0,
            humidity: 0.0,
        }
    }

    pub fn describe<L: Log>(&self, log: &mut L) {
        log.color(color::BLACK, color::WHITE);
        if !self.wall(Direction::North) { log.write("There is a path to the north.\n"); }
        if !self.wall(Direction::South) { log.write("There is a path to the south.\n"); }
        if !self.wall(Direction::West ) { log.write("There is a path to the west.\n" ); }
        if !self.wall(Direction::East ) { log.write("There is a path to the east.\n" ); }
    }

    pub fn wall(&self, wall: Direction) -> bool {
        self.flags & Self::wall_mask(wall) != 0
    }

    fn wall_mask(wall: Direction) -> u8 {
        match wall {
            Direction::North => AREA_FLAG_MASK_WALL_NORTH,
            Direction::South => AREA_FLAG_MASK_WALL_SOUTH,
            Direction::West  => AREA_FLAG_MASK_WALL_WEST,
            Direction::East  => AREA_FLAG_MASK_WALL_EAST,
        }
    }
}

impl self::maze::Cell for Area {
    fn set_wall(&mut self, wall: Direction) {
        self.flags |= Self::wall_mask(wall);
    }
}

pub mod maze;
