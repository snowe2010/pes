#![feature(plugin)]
#![plugin(cqrust_codegen)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate cqrust;


use cqrust::Crust;

#[get("/")]
fn hello() -> &'static str {
    let crust = Crust::new();
    crust.send(&"command_handler".to_string());
    "Hi!"
}

#[get("blah")]
fn boomshakalaka() -> &'static str {
    "HAHAHA"
}

#[cqrust()]
fn command_handler() {
    println!("Command handler ran! OMG I DID IT");
}


fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
    println!("Blargh")
}

//#[cqrust]
//struct BLah {
//
//}
