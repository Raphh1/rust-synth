#[derive(PartialEq, Clone, Debug)]
enum TransportState {
    Stopped,
    Playing,
}

impl TransportState {
    pub fn play(&mut self) -> Result<(), String> {
        if self.is_playing() {
            Err("Already playing".to_string())
        } else {
            *self = TransportState::Playing;
            Ok(())
        }
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if self.is_stopped() {
            Err("Already stopped".to_string())
        } else {
            *self = TransportState::Stopped;
            Ok(())
        }
    }

    pub fn is_playing(&self) -> bool {
        *self == TransportState::Playing
    }

    pub fn is_stopped(&self) -> bool {
        *self == TransportState::Stopped
    }
}