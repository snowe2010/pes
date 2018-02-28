#![feature(proc_macro)]

#[macro_use]
extern crate cqrust_codegen;
extern crate cqrust;
use cqrust::{Command, CommandGateway};

#[derive(Command)]
struct BarCommand {}

fn main() {
    println!("Hello, world!");
    CommandGateway::send(BarCommand {});
}
