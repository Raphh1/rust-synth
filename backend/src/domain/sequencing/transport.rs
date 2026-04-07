#[derive(PartialEq, Clone, Debug)]
pub enum TransportState {
    Stopped,
    Playing,
}

impl TransportState {
    pub fn play(&mut self) -> Result<(), TransportError> {
        if self.is_playing() {
            Err(TransportError::AlreadyPlaying)
        } else {
            *self = TransportState::Playing;
            Ok(())
        }
    }

    pub fn stop(&mut self) -> Result<(), TransportError> {
        if self.is_stopped() {
            Err(TransportError::AlreadyStopped)
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

#[derive(Debug)]
pub struct Transport {
    state: TransportState,
    bpm: Bpm,
    position: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bpm(f64);
impl Bpm {
   pub fn new(b: f64) -> Result<Self, TransportError> {
        if b > 0.0 { Ok(Self(b)) }
        else { Err(TransportError::InvalidBpm) }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Transport {
    pub fn new() -> Self {
        Self {
            state: TransportState::Stopped,
            bpm: Bpm(120.0),
            position: 0.0,
        }
    }

    pub fn play(&mut self) -> Result<(), TransportError> {
        self.state.play()
    }

    pub fn stop(&mut self) -> Result<(), TransportError> {
        self.state.stop()?;
        self.reset_position();
        Ok(())
    }

    pub fn set_bpm(&mut self, bpm: f64) -> Result<(), TransportError> {
        self.bpm = Bpm::new(bpm)?;
        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        self.state.is_playing()
    }

    pub fn is_stopped(&self) -> bool {
        self.state.is_stopped()
    }

    pub fn reset_position(&mut self) {
        self.position = 0.0;
    }

    pub fn bpm(&self) -> f64 {
        self.bpm.value()
    }

    pub fn position(&self) -> f64 {
        self.position
    }
}

#[derive(Debug)]
pub enum TransportError {
    AlreadyPlaying,
    AlreadyStopped,
    InvalidBpm,
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::AlreadyPlaying => write!(f, "Transport is already playing"),
            TransportError::AlreadyStopped => write!(f, "Transport is already stopped"),
            TransportError::InvalidBpm => write!(f, "BPM must be greater than 0"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_stopped() {
        let t = Transport::new();
        assert!(t.is_stopped());
        assert!(!t.is_playing());
    }

    #[test]
    fn play_transitions_to_playing() {
        let mut t = Transport::new();
        assert!(t.play().is_ok());
        assert!(t.is_playing());
    }

    #[test]
    fn stop_transitions_to_stopped() {
        let mut t = Transport::new();
        t.play().unwrap();
        assert!(t.stop().is_ok());
        assert!(t.is_stopped());
    }

    #[test]
    fn double_play_returns_error() {
        let mut t = Transport::new();
        t.play().unwrap();
        assert!(matches!(t.play(), Err(TransportError::AlreadyPlaying)));
    }

    #[test]
    fn stop_when_stopped_returns_error() {
        let mut t = Transport::new();
        assert!(matches!(t.stop(), Err(TransportError::AlreadyStopped)));
    }

    #[test]
    fn set_bpm_valid() {
        let mut t = Transport::new();
        assert!(t.set_bpm(140.0).is_ok());
        assert!((t.bpm() - 140.0).abs() < f64::EPSILON);
    }

    #[test]
    fn set_bpm_zero_returns_error() {
        let mut t = Transport::new();
        assert!(matches!(t.set_bpm(0.0), Err(TransportError::InvalidBpm)));
    }

    #[test]
    fn stop_resets_position() {
        let mut t = Transport::new();
        t.play().unwrap();
        t.stop().unwrap();
        assert!((t.position() - 0.0).abs() < f64::EPSILON);
    }
}
