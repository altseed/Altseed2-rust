#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
extern crate num;

mod auto_generated_core_binding;
pub mod engine;
pub mod math;
pub mod prelude;

pub mod structs;
#[cfg(test)]
mod tests;
