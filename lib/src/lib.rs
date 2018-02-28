#![feature(proc_macro)]

#[macro_use]
extern crate cqrust_codegen;

pub trait Command {}


//#[derive(Event)]
//struct BarEvent {
//
//}

pub struct CommandGateway {}

impl CommandGateway {
    pub fn send<T: Command>(x: T) {
        println!("Sent to command gateway");
        EventBus::receive(x)
    }
}

pub struct EventBus {}

impl EventBus {
    pub fn receive<T: Command>(x: T) {
        println!("Received in 'aggregate'");
    }
}

/// This is only here so that the crate will run as a binary crate
fn main() {
    println!("Hello, world!");
}
