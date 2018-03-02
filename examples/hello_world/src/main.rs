#![feature(proc_macro)]

#[macro_use]
extern crate cqrust_codegen;
extern crate cqrust;
#[macro_use]
extern crate lazy_static;

use cqrust::{Command, CommandGateway};
use cqrust_codegen::event_handler;
use cqrust::EventBus;

#[derive(Command)]
struct BarCommand {}

#[event_handler]
fn test(st: String) {}

fn main() {
    println!("Hello, world!");
    let eventbus = EventBus::new();
    let fun: fn(BarCommand) -> () = event_handler();
    eventbus.register(fun, 0);
    let command_gateway = CommandGateway::new(eventbus);
    command_gateway.send(BarCommand {});
}

fn event_handler(command: BarCommand) {
    println!("I'm in the event handler!!!")
}
