#![allow(warnings)]
mod application;
mod domain;
mod ipc;

fn main() {
    application::engine::run().expect("Failed to run engine");
}
