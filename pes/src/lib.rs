#![feature(proc_macro)]

#[macro_use]
extern crate pes_derive;
extern crate pes_common;
extern crate eventbus;
#[macro_use]
extern crate lazy_static;
extern crate anymap;

pub use eventbus::EventBus;
pub use pes_common::{CommandBus, Command};

use std::collections::VecDeque;

lazy_static! {
   pub static ref COMMAND_BUS: CommandBus = CommandBus::new();
}

pub struct CommandGateway {
    commandbus: &'static CommandBus
}

impl CommandGateway {
    pub fn new(commandbus: &'static CommandBus) -> CommandGateway {
        CommandGateway { commandbus }
    }

    pub fn send<T: Command>(&self, mut x: T) {
        println!("Sent to command gateway");
        self.commandbus.post(&mut x);
    }
}

fn main() {
    println!("Hello, world!");
}
