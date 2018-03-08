#![feature(proc_macro)]

//#[macro_use]
//extern crate pes_derive;
extern crate pes_common;
extern crate eventbus;
#[macro_use]
extern crate lazy_static;
extern crate anymap;

//pub use eventbus::EventBus;
pub use pes_common::command_bus::{CommandBus, Command};
pub use pes_common::event_bus::{EventBus, Event};

//use std::collections::VecDeque;

lazy_static! {
   pub static ref COMMAND_BUS: CommandBus = CommandBus::new();
   pub static ref EVENT_BUS: EventBus = EventBus::new();
}

pub struct CommandGateway {
//    commandbus: &'static CommandBus
}

impl CommandGateway {
    pub fn new() -> CommandGateway {
        CommandGateway {  }
    }

    pub fn send<T: Command>(&self, mut x: T) {
        println!("Sent to command gateway");
        COMMAND_BUS.post(&mut x);
    }
}
//
//fn main() {
//    println!("Hello, world!");
//}
