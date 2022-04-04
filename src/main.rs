#![allow(warnings)]
#![allow(dead_code)]
extern crate mpi;

use mpi::topology::SystemCommunicator;
use mpi::traits::*;
// use rand::Rng;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    match world.rank() {
        0 => {
            let mut messages: Vec<i32> = (0..world.size()).collect();
            println!("Starting message queue contains {:?}", messages);
            messages.reverse();
            println!("Rank {} got message {:?}", world.rank(), messages.pop().unwrap());
            println!("Sending message queue containing: {:?} from rank {}", messages, world.rank());
            world.process_at_rank(1).send(&messages[..]);
        }
        1 => {
            scatter_into(0, 2, &world)
        }
        2 => {
            scatter_into(1, 3, &world)
        }
        3 => {
            scatter_into(2, 0, &world)
        }
        _ => {panic!("Wrong number of ranks provided")}
    }
}

fn scatter_into(receive_from: i32, send_to: i32, world: &SystemCommunicator) {
    // let mut messages: [i32; 4]= [0,0,0,0];
    let (mut msg, status) = world.process_at_rank(receive_from).receive_vec::<i32>();
    // let (mut messages, status) = world.process_at_rank(receive_from).receive::<Vec<i32>>();
    println!("Rank {} got queue {:?}", world.rank(), msg);
    let message = msg.pop();
    println!("Rank {} poped message {:?}", world.rank(), message.unwrap());
    if msg.len() > 0{
        println!("Sending message queue containing: {:?} from rank {}", msg, world.rank());
        world.process_at_rank(send_to).send(&msg[..]);
    }

}