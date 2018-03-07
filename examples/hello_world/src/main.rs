#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pes_derive;
extern crate pes;
extern crate pes_common;

use pes::{COMMAND_BUS, CommandGateway};
use pes_common::command_bus::{Command, CommandMetadata};
use pes_common::event_bus::{Event, EventMetadata};
use std::sync::RwLock;

use pes_derive::{command_handler,event_handler,macbuild};

// this generates code to import bootstrap()
macbuild!();

#[derive(Command)]
struct BarCommand {}

#[event_handler]
fn test(st: String) {}

fn main() {
    bootstrap();

    CarAggregate::new();
    let command_gateway = CommandGateway::new();
    command_gateway.send(BarCommand {});
    command_gateway.send(BCommand { id: "hi".to_string() });
    command_gateway.send(CCommand {});
    command_gateway.send(DCommand {});
}

#[command_handler]
fn handle(_command: &mut BarCommand) {
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

#[derive(Event)]
struct DEvent {}

struct CarAggregate {}

impl CarAggregate {
    fn new() -> CarAggregate { CarAggregate {} }
    #[command_handler]
    fn handle_bar_command(_command: &mut BarCommand) {
        println!("I'm in the car aggregate bar command handler")
    }
    #[command_handler]
    fn handle_b_command(command: &mut BCommand) {
        println!("I'm in the car aggregate B command handler");
        println!("b command id is {}", command.id)
    }
    #[command_handler]
    fn handle_c_command(_command: &mut CCommand) {
        println!("I'm in the car aggregate C command handler")
    }
    #[command_handler]
    fn handle_d_command(_command: &mut DCommand) {
        println!("I'm in the car aggregate D command handler")
    }
}
