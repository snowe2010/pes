#![feature(proc_macro)]

#[macro_use]
extern crate pes_derive;
extern crate pes;
#[macro_use]
extern crate lazy_static;
extern crate pes_common;

use pes::{CommandGateway, COMMAND_BUS};
use pes_derive::event_handler;
use pes::EventBus;

use pes_common::{CommandMetadata, CommandBus, Command};
use std::sync::RwLock;
use std::collections::HashMap;

#[derive(Command)]
struct BarCommand {}

#[event_handler]
fn test(st: String) {}


fn main() {
//    let command_bus: CommandBus = CommandBus::new();
//    command_bus.register(handle, 0);
    COMMAND_BUS.register(handle, 0);
    COMMAND_BUS.register(CarAggregate::handle_bar_command, 0);
    COMMAND_BUS.register(CarAggregate::handle_b_command, 0);
    COMMAND_BUS.register(CarAggregate::handle_c_command, 0);
    COMMAND_BUS.register(CarAggregate::handle_d_command, 0);

//    let command_gateway = CommandGateway::new(command_bus);
    let command_gateway = CommandGateway::new(&COMMAND_BUS);
    command_gateway.send(BarCommand {});
    command_gateway.send(BCommand { id: "hi".to_string() });
    command_gateway.send(CCommand {});
    command_gateway.send(DCommand {});

}

fn handle(command: &mut BarCommand) {
    println!("I'm in the event handler!!!")
}


#[derive(Command)]
struct BCommand {
    id: String
}
#[derive(Command)]
struct CCommand {}
#[derive(Command)]
struct DCommand {}

struct CarAggregate {}
impl CarAggregate {
    fn handle_bar_command(command: &mut BarCommand) {
        println!("I'm in the car aggregate bar command handler")
    }
    fn handle_b_command(command: &mut BCommand) {
        println!("I'm in the car aggregate B command handler");
        println!("b command id is {}", command.id)
    }
    fn handle_c_command(command: &mut CCommand) {
        println!("I'm in the car aggregate C command handler")
    }
    fn handle_d_command(command: &mut DCommand) {
        println!("I'm in the car aggregate D command handler")
    }
}

trait Aggregate {
    fn register<T>(&'static self, fun_list: HashMap<HashEq<fn(&mut T)>, i32>) where T: Command {
        for (function, priority) in fun_list {
            COMMAND_BUS.register(function.0, priority);
        }
    }
}

use std::hash::{Hash, Hasher};
struct HashEq<T>(fn(&mut T));

impl<T> PartialEq for HashEq<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}

impl<T> Eq for HashEq<T> {}

impl<T> Hash for HashEq<T> {
    fn hash<H>(&self, state: &mut H)
        where
            H: Hasher
    {
        state.write_usize(self.0 as usize)
    }
}
