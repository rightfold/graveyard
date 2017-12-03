use infra::common::direction::Direction;
use infra::common::protocol::Request;

pub fn parse_command(command: &str) -> Option<Request> {
    match command {
        "look"  => Some(Request::Look),
        "north" => Some(Request::Move(Direction::North)),
        "south" => Some(Request::Move(Direction::South)),
        "west"  => Some(Request::Move(Direction::West )),
        "east"  => Some(Request::Move(Direction::East )),
        _ => None,
    }
}
