#[derive(Clone, Debug, PartialEq)]
pub struct NoteEvent {
    id: NoteId,
    pitch: Pitch,
    start: Start,
    length: Length,
    velocity: Velocity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity(f64);
impl Velocity {
    pub fn new(velocity: f64) -> Result<Self, NoteEventError> {
        if (0.0..=1.0).contains(&velocity) {
            Ok(Self(velocity))
        } else {
            Err(NoteEventError::InvalidVelocity)
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pitch(u8);
impl Pitch {
    pub fn new(p: u8) -> Result<Self, NoteEventError> {
        if p <= 127 {
            Ok(Self(p))
        } else {
            Err(NoteEventError::InvalidPitch)
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Start(f64);
impl Start {
    pub fn new(s: f64) -> Result<Self, NoteEventError> {
        if s >= 0.0 {
            Ok(Self(s))
        } else {
            Err(NoteEventError::InvalidStart)
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Length(f64);
impl Length {
    pub fn new(l: f64) -> Result<Self, NoteEventError> {
        if l > 0.0 {
            Ok(Self(l))
        } else {
            Err(NoteEventError::InvalidLength)
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NoteId(String);
impl NoteId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum NoteEventError {
    InvalidVelocity,
    InvalidPitch,
    InvalidLength,
    InvalidStart,
}

impl NoteEvent {
    fn new(
        id: NoteId,
        pitch: u8,
        start: f64,
        length: f64,
        velocity: f64,
    ) -> Result<Self, PatternError> {
        let pitch = Pitch::new(pitch)?;
        let start = Start::new(start)?;
        let length = Length::new(length)?;
        let velocity = Velocity::new(velocity)?;

        Ok(Self {
            id,
            pitch,
            start,
            length,
            velocity,
        })
    }

    pub(crate) fn set_pitch(&mut self, pitch: u8) -> Result<(), NoteEventError> {
        self.pitch = Pitch::new(pitch)?;
        Ok(())
    }

    pub(crate) fn set_start(&mut self, start: f64) -> Result<(), NoteEventError> {
        self.start = Start::new(start)?;
        Ok(())
    }

    pub(crate) fn set_length(&mut self, length: f64) -> Result<(), NoteEventError> {
        self.length = Length::new(length)?;
        Ok(())
    }

    pub(crate) fn set_velocity(&mut self, velocity: f64) -> Result<(), NoteEventError> {
        self.velocity = Velocity::new(velocity)?;
        Ok(())
    }

    pub fn end(&self) -> f64 {
        self.start() + self.length()
    }

    pub fn id(&self) -> &str {
        &self.id.0
    }

    pub fn pitch(&self) -> u8 {
        self.pitch.value()
    }

    pub fn start(&self) -> f64 {
        self.start.value()
    }

    pub fn length(&self) -> f64 {
        self.length.value()
    }

    pub fn velocity(&self) -> f64 {
        self.velocity.value()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    notes: Vec<NoteEvent>,
    next_note_id: u64,
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            next_note_id: 0,
        }
    }

    pub fn add_note(
        &mut self,
        pitch: u8,
        start: f64,
        length: f64,
        velocity: f64,
    ) -> Result<NoteId, PatternError> {
        let id = NoteId(self.next_note_id.to_string());
        self.next_note_id += 1;
        self.notes
            .push(NoteEvent::new(id.clone(), pitch, start, length, velocity)?);
        Ok(id)
    }

    pub fn move_note(&mut self, id: &NoteId, pitch: u8, start: f64) -> Result<(), PatternError> {
        if let Some(note) = self.notes.iter_mut().find(|n| &n.id == id) {
            note.set_pitch(pitch)?;
            note.set_start(start)?;
            Ok(())
        } else {
            Err(PatternError::NoteNotFound)
        }
    }

    pub fn resize_note(&mut self, id: &NoteId, length: f64) -> Result<(), PatternError> {
        if let Some(note) = self.notes.iter_mut().find(|n| &n.id == id) {
            note.set_length(length)?;
            Ok(())
        } else {
            Err(PatternError::NoteNotFound)
        }
    }

    pub fn delete_note(&mut self, id: &NoteId) -> Result<(), PatternError> {
        if let Some(pos) = self.notes.iter().position(|n| &n.id == id) {
            self.notes.remove(pos);
            Ok(())
        } else {
            Err(PatternError::NoteNotFound)
        }
    }

    pub fn active_notes_at(&self, time: f64) -> Vec<&NoteEvent> {
        self.notes
            .iter()
            .filter(|n| n.start() <= time && time < n.end())
            .collect()
    }

    pub fn notes(&self) -> &[NoteEvent] {
        &self.notes
    }
}

impl From<NoteEventError> for PatternError {
    fn from(err: NoteEventError) -> Self {
        match err {
            NoteEventError::InvalidVelocity => PatternError::InvalidVelocity,
            NoteEventError::InvalidPitch => PatternError::InvalidPitch,
            NoteEventError::InvalidLength => PatternError::InvalidLength,
            NoteEventError::InvalidStart => PatternError::InvalidStart,
        }
    }
}

#[derive(Debug)]
pub enum PatternError {
    NoteNotFound,
    InvalidVelocity,
    InvalidPitch,
    InvalidLength,
    InvalidStart,
}

impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternError::NoteNotFound => write!(f, "Note not found"),
            PatternError::InvalidVelocity => {
                write!(f, "Invalid velocity (must be between 0.0 and 1.0)")
            }
            PatternError::InvalidPitch => write!(f, "Invalid pitch (must be between 0 and 127)"),
            PatternError::InvalidLength => write!(f, "Invalid length (must be greater than 0)"),
            PatternError::InvalidStart => write!(f, "Invalid start time (must be non-negative)"),
        }
    }
}
