mod ipc;
mod domain;
mod application;

fn main() {
    ipc::run().expect("Failed to run IPC server");
}
