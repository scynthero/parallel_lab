#![deny(warnings)]
extern crate mpi;

use mpi::topology::SystemCommunicator;
use mpi::traits::*;
use rand::Rng;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    match world.rank() {
        0 => {
            let number_to_send: i64 = rand::thread_rng().gen_range(0..100);
            println!("Sending number {} from rank {}", number_to_send, world.rank());
            world.process_at_rank(1).send(&number_to_send);
            let mut buf: i64 = 0;
            world.process_at_rank(3).receive_into(&mut buf);
            println!("Rank {} received number: {}", world.rank(), buf);
        }
        1 => {
            send_receive(0, 2, &world)
        }
        2 => {
            send_receive(1, 3, &world)
        }
        3 => {
            send_receive(2, 0, &world)
        }
        _ => {panic!("Wrong number of ranks provided")}
    }
}

fn send_receive(receive_from: i32, send_to: i32, world: &SystemCommunicator) {
    let mut buf: i64 = 0;
    world.process_at_rank(receive_from).receive_into(&mut buf);
    println!("Rank {} received number: {}", world.rank(), buf);
    let number_to_add = rand::thread_rng().gen_range(0..100);
    println!("Rank {} adds: {}", world.rank(), number_to_add);
    buf += number_to_add;
    println!("Sending number {} from rank {}", buf, world.rank());
    world.process_at_rank(send_to).send(&buf);
}