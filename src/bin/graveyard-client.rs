extern crate bear_lib_terminal;
extern crate bincode;
extern crate graveyard;

use bear_lib_terminal::terminal;

use graveyard::game::client::command::parse_command;
use graveyard::game::client::dialogue::describe_area;
use graveyard::infra::client::color;
use graveyard::infra::client::log::Log;
use graveyard::infra::client::log;
use graveyard::infra::client::prompt::Prompt;
use graveyard::infra::client::prompt;
use graveyard::infra::common::protocol::{Request, Response, receive_response, send_request};

use std::fmt::Debug;
use std::io;
use std::net::TcpStream;

fn main() {
    let mut server = TcpStream::connect("127.0.0.1:1337").unwrap();
    println!("connect {:?}", server);

    terminal::open("Graveyard", 140, 40);

    let mut log = log::BearLibTerminal::new(0, 60, 0, 19);
    let mut prompt = prompt::BearLibTerminal::new(0, 60, 20);

    loop {
        let command = prompt.accept();
        log.write("> ");
        log.write(&command);
        log.write("\n");

        match parse_command(&command) {
            None => {
                log.color(color::BLACK, color::RED);
                log.write("I do not understand that!\n");
            },
            Some(request) =>
                match exchange(&mut server, &request).unwrap() {
                    Response::Area(area) => describe_area(&mut log, &area),
                },
        }
    }
}

fn exchange<RW>(rw: &mut RW, request: &Request) -> bincode::Result<Response>
    where RW: Debug + io::Read + io::Write {
    println!("request {:?} {:?}", rw, request);
    send_request(rw, request)?;
    let response = receive_response(rw)?;
    println!("response {:?} {:?}", rw, response);
    Ok(response)
}
