use game::common::area::Area;
use infra::client::color;
use infra::client::log::Log;
use infra::common::direction::Direction;

pub fn describe_area<L: Log>(log: &mut L, area: &Area) {
    log.color(color::BLACK, color::WHITE);
    if !area.wall(Direction::North) { log.write("There is a path to the north.\n"); }
    if !area.wall(Direction::South) { log.write("There is a path to the south.\n"); }
    if !area.wall(Direction::West ) { log.write("There is a path to the west.\n" ); }
    if !area.wall(Direction::East ) { log.write("There is a path to the east.\n" ); }
}
