#![feature(proc_macro)]

#[macro_use]
extern crate cqrust_codegen;

use cqrust_codegen::not_the_bees;
use cqrust_codegen::Event;


/// This one shouldn't raise any errors because it doesn't have a field
/// named "bees", or any named fields at all for that matter.
#[not_the_bees]
struct Foo(i32);

/// This is where the action will happen.
#[not_the_bees]
struct Bar {
    baz: i32,
//    bees: String,
}

#[derive(Event)]
struct BarEvent {

}

/// This is only here so that the crate will run as a binary crate
fn main() {
    println!("Hello, world!");
}
