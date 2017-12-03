use std::thread::sleep;
use std::time::Duration;

use bear_lib_terminal::terminal;

use color::Color;

pub trait Log {
    fn color(&mut self, Color, Color);
    fn write(&mut self, &str);
}

pub struct BearLibTerminal {
    x1: i32,
    x2: i32,
    _y1: i32,
    _y2: i32,

    x: i32,
    y: i32,
}

impl BearLibTerminal {
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        BearLibTerminal{x1, x2, _y1: y1, _y2: y2, x: x1, y: y1}
    }
}

impl Log for BearLibTerminal {
    fn color(&mut self, background: Color, foreground: Color) {
        terminal::set_colors(foreground, background);
    }

    fn write(&mut self, message: &str) {
        for c in message.chars() {
            if c != '\n' {
                if let None = terminal::peek_event() {
                    sleep(Duration::new(0, 7_000_000));
                }
                terminal::put_xy(self.x, self.y, c);
                terminal::refresh();
                self.x += 1;
            }
            if self.x == self.x2 || c == '\n' {
                self.y += 1;
                self.x = self.x1;
            }
        }
    }
}
