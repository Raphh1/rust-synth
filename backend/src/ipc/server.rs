use std::io::{self, BufRead, Write};
use crate::ipc::protocol::{Command, ErrorCode, Response};



struct IpcServer {
    transport_state: TransportState,
}

impl IpcServer {
    fn new() -> Self {
        Self {
            transport_state: TransportState::Stopped,
        }
    }

    fn handle(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::Play { request_id } => {
                if self.transport_state == TransportState::Playing {
                    Response::Error {
                        request_id,
                        code: ErrorCode::EditDeniedPlaying,
                        message: "Already playing".to_string(),
                    }
                } else {
                    self.transport_state = TransportState::Playing;
                    Response::Ok { request_id }
                }
            }
            Command::Stop { request_id } => {
                if self.transport_state == TransportState::Stopped {
                    Response::Error {
                        request_id,
                        code: ErrorCode::EditDeniedPlaying,
                        message: "Already stopped".to_string(),
                    }
                } else {
                    self.transport_state = TransportState::Stopped;
                    Response::Ok { request_id }
                }
            }
            _ => Response::Error {
                request_id: "unknown".to_string(),
                code: ErrorCode::UnknownCommand,
                message: "Unknown command".to_string(),
            },
        }
    }
}
    


pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut server = IpcServer::new();

    for line in stdin.lock().lines() {
        let line = line?;
        match serde_json::from_str::<Command>(&line) {
            Ok(cmd) => {
                // Handle the command and generate a response
                let response = server.handle(cmd);
                // Send the response back to the client
                let response_json = serde_json::to_string(&response).unwrap();
                writeln!(stdout, "{}", response_json)?;
                stdout.flush()?;
            }
            Err(_) => {
                // Send an error response for invalid JSON
                let error_response = Response::Error {
                    request_id: "unknown".to_string(),
                    code: ErrorCode::InvalidMessage,
                    message: "Invalid JSON".to_string(),
                };
                let error_json = serde_json::to_string(&error_response).unwrap();
                writeln!(stdout, "{}", error_json)?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}

#[derive(PartialEq)]
enum TransportState {
    Stopped,
    Playing,
}
