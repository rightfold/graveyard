extern crate bear_lib_terminal;
extern crate graveyard;
extern crate rand;

use graveyard::area::Area;
use graveyard::color;
use graveyard::log::Log;
use graveyard::log;
use graveyard::matrix::Matrix;
use graveyard::maze;
use graveyard::prompt::Prompt;
use graveyard::prompt;

use bear_lib_terminal::terminal;

fn main() {
    let mut rng = rand::thread_rng();
    let mut world = Matrix::new(Area::new(), 128, 128);
    maze::generate(&mut rng, &mut world);

    terminal::open("Graveyard", 140, 40);

    let mut log = log::BearLibTerminal::new(0, 60, 0, 19);
    let mut prompt = prompt::BearLibTerminal::new(0, 60, 20);

    let mut x = 0;
    let mut y = 0;
    loop {
        world[(x, y)].describe(&mut log);

        let command = prompt.accept();
        log.write("> ");
        log.write(&command);
        log.write("\n");
        match &command as &str {
            "north" => y -= 1,
            "south" => y += 1,
            "west"  => x -= 1,
            "east"  => x += 1,
            _ => {
                log.color(color::BLACK, color::RED);
                log.write("I do not understand that!\n");
            },
        }
    }
}
