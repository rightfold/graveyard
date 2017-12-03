use infra::client::color;

use bear_lib_terminal::geometry::Point;
use bear_lib_terminal::terminal;

pub trait Prompt {
    fn accept(&mut self) -> String;
}

pub struct BearLibTerminal {
    x1: i32,
    x2: i32,
    y: i32,
}

impl BearLibTerminal {
    pub fn new(x1: i32, x2: i32, y: i32) -> Self {
        BearLibTerminal{x1, x2, y}
    }
}

impl Prompt for BearLibTerminal {
    fn accept(&mut self) -> String {
        terminal::set_colors(color::WHITE, color::BLACK);
        terminal::print_xy(self.x1, self.y, "> ");
        let point = Point::new(self.x1 + 2, self.y);
        match terminal::read_str(point, self.x2 - point.x) {
            Some(result) => result,
            None => self.accept(),
        }
    }
}
