use infra::common::direction::Direction;
use infra::common::protocol::Request;

pub fn parse_command(command: &str) -> Option<Request> {
    match command.trim() {
        "l" | "look"  => Some(Request::Look),
        "n" | "north" => Some(Request::Move(Direction::North)),
        "s" | "south" => Some(Request::Move(Direction::South)),
        "w" | "west"  => Some(Request::Move(Direction::West )),
        "e" | "east"  => Some(Request::Move(Direction::East )),
        _ => None,
    }
}
