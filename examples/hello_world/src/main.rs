#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate pes;
extern crate pes_common;
#[macro_use]
extern crate pes_derive;

use pes::{COMMAND_BUS, CommandGateway};
use pes::EventBus;
use pes_common::{Command, CommandBus, CommandMetadata};
use pes_derive::event_handler;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Command)]
struct BarCommand {}

#[event_handler]
fn test(st: String) {}


fn main() {
    CarAggregate::new();
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
    fn new() -> CarAggregate {
        COMMAND_BUS.register(CarAggregate::handle_bar_command, 0);
        COMMAND_BUS.register(CarAggregate::handle_b_command, 0);
        COMMAND_BUS.register(CarAggregate::handle_c_command, 0);
        COMMAND_BUS.register(CarAggregate::handle_d_command, 0);

        CarAggregate {}
    }
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

//trait Aggregate {
//    fn register<T>(&'static self, fun_list: HashMap<HashEq<T>, i32>) where T: Command {
//    fn register<T>(fun_list: HashMap<HashEq<T>, i32>) where T: Command {
//        for (function, priority) in fun_list {
//            COMMAND_BUS.register(function.0, priority);
//        }
//    }
//}
