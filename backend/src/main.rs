mod ipc;
mod domain;

fn main() {
    ipc::run().expect("Failed to run IPC server");
}
