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
