#[derive(PartialEq, Clone, Debug)]
enum TransportState {
    Stopped,
    Playing,
}

impl TransportState {
    fn play(&mut self) -> Result<(), String> {
        if self.is_playing() {
            Err("Already playing".to_string())
        } else {
            *self = TransportState::Playing;
            Ok(())
        }
    }

    fn stop(&mut self) -> Result<(), String> {
        if self.is_stopped() {
            Err("Already stopped".to_string())
        } else {
            *self = TransportState::Stopped;
            Ok(())
        }
    }

    fn is_playing(&self) -> bool {
        *self == TransportState::Playing
    }
    
    fn is_stopped(&self) -> bool {
            *self == TransportState::Stopped
    }
}