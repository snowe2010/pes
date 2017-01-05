pub mod crust;
pub use crust::Crust;
mod codegen;
mod handler;
pub use codegen::{CommandGatewayHandlerInfo};
pub fn send() {
    Crust::send()
}
