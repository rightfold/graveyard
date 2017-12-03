extern crate bear_lib_terminal;
extern crate graveyard;
extern crate rand;

use graveyard::area::Area;
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

    terminal::open("Graveyard", 80, 24);

    let mut log = log::BearLibTerminal::new(0, 40, 0, 10);
    let mut prompt = prompt::BearLibTerminal::new(0, 40, 11);

    world[(10, 10)].describe(&mut log);

    loop {
        let command = prompt.accept();
        log.write("> ");
        log.write(&command);
        log.write("\n");
    }
}
