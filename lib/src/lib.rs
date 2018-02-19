#![feature(quote, concat_idents, plugin_registrar, rustc_private, unicode)]

#[macro_use] extern crate log;
extern crate term_painter;

mod crust;
pub use crust::Crust;
mod codegen;
mod router;
pub mod handler;

pub use router::Route;

#[doc(inline)] pub use handler::{Handler};

pub use codegen::{CommandGatewayHandlerInfo};
pub fn send(name: &String) {
    Crust::new().send(name)
}
