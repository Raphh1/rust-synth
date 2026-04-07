mod application;
mod domain;
mod ipc;

fn main() {
    ipc::run().expect("Failed to run IPC server");
}
