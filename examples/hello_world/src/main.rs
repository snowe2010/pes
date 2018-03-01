#![feature(proc_macro)]

#[macro_use]
extern crate cqrust_codegen;
extern crate cqrust;
use cqrust::{Command, CommandGateway};
use cqrust_codegen::event_handler;

#[derive(Command)]
struct BarCommand {}

#[event_handler]
fn test(st: String) {}

fn main() {
    println!("Hello, world!");
    CommandGateway::send(BarCommand {});
}
