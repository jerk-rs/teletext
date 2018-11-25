#![feature(box_patterns)]
extern crate teleborg;

#[macro_use]
extern crate failure;

mod app;
mod transform;

pub use self::app::run;
