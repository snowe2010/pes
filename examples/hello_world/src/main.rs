#![feature(plugin)]
#![plugin(CQRuSt_codegen)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate CQRuSt;

#[get("/")]
fn hello() -> &'static str {
    CQRuSt::Crust::send();
    "Hi!"
}

#[get("blah")]
fn boomshakalaka() -> &'static str {
    "HAHAHA"
}

#[CQRuSt]
fn command_handler() {
    println!("Command handler ran! OMG I DID IT");
}


fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
    println!("Blargh")
}

//#[CQRuSt]
//struct BLah {
//
//}
