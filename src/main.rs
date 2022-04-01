#![deny(warnings)]
extern crate mpi;

use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let receiver_rank = 1;

    if world.rank() == receiver_rank {
        let mut buf: i64 = 0;
        world.any_process().receive_into(&mut buf);
        println!("Rank {} received number: {}", world.rank(), buf);
    } else {
        let number_to_send = 15;
        println!("Sending number {} from rank {}", number_to_send, world.rank());
        world.process_at_rank(1).send(&number_to_send);
    }
}