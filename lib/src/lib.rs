#![feature(proc_macro)]

#[macro_use]
extern crate cqrust_codegen;
extern crate eventbus;
extern crate uuid;
#[macro_use]
extern crate lazy_static;

mod command_bus;

pub use eventbus::EventBus;
pub use command_bus::{CommandBus, Command};

use std::collections::VecDeque;

pub struct CommandGateway {
    commandbus: CommandBus
}

impl CommandGateway {
    pub fn new(commandbus: CommandBus) -> CommandGateway {
        CommandGateway { commandbus }
    }

    pub fn send<T: Command>(&self, mut x: T) {
        println!("Sent to command gateway");
//        self.bus.vector.a
        self.commandbus.post(&mut x);
    }
}

//pub struct EventBus {}

//impl EventBus {
//    pub fn receive<T: Command>(x: T) {
//        println!("Received in 'aggregate'");
//    }
//}

fn main() {
    println!("Hello, world!");
}
