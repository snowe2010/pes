#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pes_derive;
extern crate pes;
extern crate pes_common;

use pes::{COMMAND_BUS, CommandGateway, EVENT_BUS};
use pes_common::command_bus::{Command, CommandMetadata};
use pes_common::event_bus::{Event, EventMetadata};
use std::sync::RwLock;

use pes_derive::{command_handler,event_handler,macbuild};

// this generates code to import bootstrap()
macbuild!();

fn main() {
    bootstrap();

    CarAggregate::new();
    let command_gateway = CommandGateway::new();
    command_gateway.send(ACommand { id: "A".to_string() });
    command_gateway.send(BCommand { id: "B".to_string() });
    command_gateway.send(CCommand { id: "C".to_string() });
    command_gateway.send(DCommand { id: "D".to_string() });
}

#[command_handler]
fn handle(_command: &mut ACommand) {
    println!("I'm in a top level event handler!!!")
}

#[derive(Command)]
struct ACommand { id: String }
#[derive(Command)]
struct BCommand { id: String }
#[derive(Command)]
struct CCommand { id: String }
#[derive(Command)]
struct DCommand { id: String }

#[derive(Event)]
struct AEvent { id: String }
#[derive(Event)]
struct BEvent { id: String }
#[derive(Event)]
struct CEvent { id: String }
#[derive(Event)]
struct DEvent { id: String }

struct CarAggregate {}

impl CarAggregate {
    fn new() -> CarAggregate { CarAggregate {} }
    #[command_handler]
    fn handle_bar_command(_command: &mut ACommand) {
        println!("I'm in the car aggregate bar command handler");
    }
    #[command_handler]
    fn handle_b_command(command: &mut BCommand) {
        println!("I'm in the car aggregate B command handler");
        println!("b command id is {}", command.id);
        apply(BEvent{ id: command.id.clone() })
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

struct CarProjector {}
impl CarProjector {
    fn new() -> CarProjector { CarProjector {} }

    #[event_handler]
    fn handle_bar_event(_event: &mut AEvent) {
        println!("I'm in the car projector bar event handler");
    }
    #[event_handler]
    fn handle_b_event(event: &mut BEvent) {
        println!("I'm in the car projector B event handler");
        println!("b event id is {}", event.id);
    }
    #[event_handler]
    fn handle_c_event(_event: &mut CEvent) {
        println!("I'm in the car projector C event handler")
    }
    #[event_handler]
    fn handle_d_event(_event: &mut DEvent) {
        println!("I'm in the car projector D event handler")
    }
}

fn apply<T: Event>(mut event: T) {
    println!("Event applied");
    EVENT_BUS.post(&mut event);
}

