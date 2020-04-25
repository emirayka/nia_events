#[macro_use]
extern crate lazy_static;
extern crate uinput_sys;
extern crate uinput;
extern crate xcb;

mod utils;
mod input_listeners;
mod output_senders;

pub use {
    utils::*,
    input_listeners::*,
    output_senders::*,
};

