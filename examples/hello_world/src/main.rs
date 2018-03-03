#![feature(proc_macro)]

#[macro_use]
extern crate cqrust_codegen;
extern crate cqrust;
#[macro_use]
extern crate lazy_static;
extern crate pes_common;

use cqrust::{CommandGateway, COMMAND_BUS};
use cqrust_codegen::event_handler;
use cqrust::EventBus;

use pes_common::{CommandMetadata, CommandBus, Command};
use std::sync::RwLock;

#[derive(Command)]
struct BarCommand {}

#[event_handler]
fn test(st: String) {}


fn main() {
//    let command_bus: CommandBus = CommandBus::new();
//    command_bus.register(handle, 0);
    COMMAND_BUS.register(handle, 0);
//    let command_gateway = CommandGateway::new(command_bus);
    let command_gateway = CommandGateway::new(&COMMAND_BUS);
    command_gateway.send(BarCommand {});
}

fn handle(command: &mut BarCommand) {
    println!("I'm in the event handler!!!")
}
