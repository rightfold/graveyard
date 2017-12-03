extern crate bear_lib_terminal;
extern crate graveyard;

use bear_lib_terminal::terminal;

fn main() {
    terminal::open("Graveyard", 80, 24);

    terminal::print_xy(0, 0, "[color=red]Hello, [color=blue]world[color=red]!");

    terminal::refresh();

    let _ = terminal::wait_event();

    terminal::close();
}
