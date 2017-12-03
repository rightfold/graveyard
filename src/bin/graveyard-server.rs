extern crate graveyard;
extern crate rand;

use graveyard::game::common::area::Area;
use graveyard::game::server::maze;
use graveyard::infra::common::direction::Direction;
use graveyard::infra::common::matrix::Matrix;
use graveyard::infra::common::protocol::{Request, Response, receive_request, send_response};

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut rng = rand::thread_rng();

    let world = Arc::new(Mutex::new({
        let mut world = Matrix::new(Area::new(), 128, 128);
        maze::generate(&mut rng, &mut world);
        world
    }));

    let listener = TcpListener::bind("0.0.0.0:1337").unwrap();
    println!("listen {:?}", listener);
    for client in listener.incoming().map(Result::unwrap) {
        let world_ref = world.clone();
        println!("accept {:?}", client);
        thread::spawn(move || {
            serve(client, &world_ref);
        });
    }
}

fn serve(mut client: TcpStream, world_mutex: &Mutex<Matrix<Area>>) {
    let mut area_position = (0, 0);

    loop {
        let request = receive_request(&mut client).unwrap();
        println!("request {:?} {:?}", client, request);

        let response = {
            let world = world_mutex.lock().unwrap();
            match request {
                Request::Look => area_response(&world, area_position),
                Request::Move(Direction::North) => {
                    area_position.0 -= 1;
                    area_response(&world, area_position)
                },
                Request::Move(Direction::South) => {
                    area_position.0 += 1;
                    area_response(&world, area_position)
                },
                Request::Move(Direction::West) => {
                    area_position.1 -= 1;
                    area_response(&world, area_position)
                },
                Request::Move(Direction::East) => {
                    area_position.1 += 1;
                    area_response(&world, area_position)
                },
            }
        };

        send_response(&mut client, &response).unwrap();
        println!("response {:?} {:?}", client, response);
    }
}

fn area_response(world: &Matrix<Area>, position: (usize, usize)) -> Response {
    Response::Area(world[position].clone())
}
