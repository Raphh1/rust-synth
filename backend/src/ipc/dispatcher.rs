use crate::application::export::export_wav;
use crate::domain::sequencing::pattern::Pattern;
use crate::domain::sequencing::transport::Transport;
use crate::domain::synthesis::lfo::{LfoShape, LfoTarget};
use crate::domain::synthesis::wavetable::Wavetable;
use crate::ipc::protocol::{Command, ErrorCode, Response};

/// Etat partag entre le dispatcher (thread IPC) et l'engine audio.
/// Les paramtres DSP sont stocks ici ; l'audio thread les lit  chaque buffer.
pub struct EngineState {
    pub transport: Transport,
    pub pattern: Pattern,
    pub wavetable: Wavetable,
    pub loop_enabled: bool,

    // Envelope ADSR
    pub envelope_attack: f64,
    pub envelope_decay: f64,
    pub envelope_sustain: f64,
    pub envelope_release: f64,

    // Filtre
    pub filter_cutoff: f64,
    pub filter_resonance: f64,

    // LFO
    pub lfo_rate: f64,
    pub lfo_depth: f64,
    pub lfo_shape: LfoShape,
    pub lfo_target: LfoTarget,

    // Portamento
    pub portamento_time: f64,

    // Sample rate du device audio (fixé au démarrage)
    pub sample_rate: f64,
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            transport: Transport::new(),
            pattern: Pattern::new(),
            wavetable: Wavetable::sine(Wavetable::DEFAULT_SIZE),
            loop_enabled: false,
            envelope_attack: 0.01,
            envelope_decay: 0.1,
            envelope_sustain: 0.7,
            envelope_release: 0.3,
            filter_cutoff: 1000.0,
            filter_resonance: 0.5,
            lfo_rate: 1.0,
            lfo_depth: 0.5,
            lfo_shape: LfoShape::Sine,
            lfo_target: LfoTarget::Cutoff,
            portamento_time: 0.0,
            sample_rate: 44100.0,
        }
    }
}

/// Dispatche une commande IPC vers l'tat du moteur.
pub fn dispatch(state: &mut EngineState, cmd: Command) -> Response {
    match cmd {
        Command::Play { request_id } => {
            match state.transport.play() {
                Ok(_) => Response::Ok { request_id },
                Err(e) => Response::Error {
                    request_id,
                    code: ErrorCode::EditDeniedPlaying,
                    message: e.to_string(),
                },
            }
        }

        Command::Stop { request_id } => {
            match state.transport.stop() {
                Ok(_) => Response::Ok { request_id },
                Err(e) => Response::Error {
                    request_id,
                    code: ErrorCode::EditDeniedPlaying,
                    message: e.to_string(),
                },
            }
        }

        Command::SetParam { request_id, name, value } => {
            match set_param(state, &name, value) {
                Ok(_) => Response::Ok { request_id },
                Err(msg) => Response::Error {
                    request_id,
                    code: ErrorCode::ParamNotFound,
                    message: msg,
                },
            }
        }

        Command::AddNote { request_id, note } => {
            match state.pattern.add_note(note.id, note.pitch, note.start, note.length, note.velocity) {
                Ok(_) => Response::Ok { request_id },
                Err(e) => Response::Error {
                    request_id,
                    code: ErrorCode::InternalError,
                    message: e.to_string(),
                },
            }
        }

        Command::MoveNote { request_id, id, pitch, start } => {
            match state.pattern.move_note_by_str(&id, pitch, start) {
                Ok(_) => Response::Ok { request_id },
                Err(e) => Response::Error {
                    request_id,
                    code: ErrorCode::InternalError,
                    message: e.to_string(),
                },
            }
        }

        Command::ResizeNote { request_id, id, length } => {
            match state.pattern.resize_note_by_str(&id, length) {
                Ok(_) => Response::Ok { request_id },
                Err(e) => Response::Error {
                    request_id,
                    code: ErrorCode::InternalError,
                    message: e.to_string(),
                },
            }
        }

        Command::DeleteNote { request_id, id } => {
            match state.pattern.delete_note_by_str(&id) {
                Ok(_) => Response::Ok { request_id },
                Err(e) => Response::Error {
                    request_id,
                    code: ErrorCode::InternalError,
                    message: e.to_string(),
                },
            }
        }

        Command::WavetableSet { request_id, table, .. } => {
            state.wavetable = Wavetable::from_samples(table);
            Response::Ok { request_id }
        }

        Command::LoopToggle { request_id, enabled } => {
            state.loop_enabled = enabled;
            Response::Ok { request_id }
        }

        Command::ExportWav { request_id, path } => {
            match export_wav(state, &path) {
                Ok(_) => Response::Ok { request_id },
                Err(e) => Response::Error {
                    request_id,
                    code: ErrorCode::ExportFailed,
                    message: e,
                },
            }
        }

        Command::PatchReplace { request_id, .. } => {
            Response::Ok { request_id }
        }

        Command::PatchValidate { request_id } => {
            Response::Ok { request_id }
        }
    }
}

fn set_param(state: &mut EngineState, name: &str, value: f64) -> Result<(), String> {
    match name {
        "transport.bpm"      => state.transport.set_bpm(value).map_err(|e| e.to_string()),
        "envelope.attack"    => { state.envelope_attack  = value.max(0.001); Ok(()) }
        "envelope.decay"     => { state.envelope_decay   = value.max(0.001); Ok(()) }
        "envelope.sustain"   => { state.envelope_sustain = value.clamp(0.0, 1.0); Ok(()) }
        "envelope.release"   => { state.envelope_release = value.max(0.001); Ok(()) }
        "filter.cutoff"      => { state.filter_cutoff    = value.clamp(20.0, 20000.0); Ok(()) }
        "filter.resonance"   => { state.filter_resonance = value.clamp(0.0, 1.0); Ok(()) }
        "lfo.rate"           => { state.lfo_rate         = value.max(0.01); Ok(()) }
        "lfo.depth"          => { state.lfo_depth        = value.clamp(0.0, 1.0); Ok(()) }
        "lfo.shape"          => { state.lfo_shape        = LfoShape::from_f64(value); Ok(()) }
        "lfo.target"         => { state.lfo_target       = LfoTarget::from_f64(value); Ok(()) }
        "portamento.time"    => { state.portamento_time  = value.max(0.0); Ok(()) }
        _ => Err(format!("Unknown param: {}", name)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> EngineState { EngineState::new() }

    #[test]
    fn test_play_then_stop() {
        let mut s = state();
        let r = dispatch(&mut s, Command::Play { request_id: "1".into() });
        assert!(matches!(r, Response::Ok { .. }));
        assert!(s.transport.is_playing());

        let r = dispatch(&mut s, Command::Stop { request_id: "2".into() });
        assert!(matches!(r, Response::Ok { .. }));
        assert!(s.transport.is_stopped());
    }

    #[test]
    fn test_double_play_returns_error() {
        let mut s = state();
        dispatch(&mut s, Command::Play { request_id: "1".into() });
        let r = dispatch(&mut s, Command::Play { request_id: "2".into() });
        assert!(matches!(r, Response::Error { .. }));
    }

    #[test]
    fn test_set_param_filter_cutoff() {
        let mut s = state();
        dispatch(&mut s, Command::SetParam {
            request_id: "1".into(), name: "filter.cutoff".into(), value: 5000.0,
        });
        assert_eq!(s.filter_cutoff, 5000.0);
    }

    #[test]
    fn test_set_param_unknown_returns_error() {
        let mut s = state();
        let r = dispatch(&mut s, Command::SetParam {
            request_id: "1".into(), name: "unknown.param".into(), value: 1.0,
        });
        assert!(matches!(r, Response::Error { .. }));
    }

    #[test]
    fn test_add_and_delete_note() {
        let mut s = state();
        dispatch(&mut s, Command::AddNote {
            request_id: "1".into(),
            note: crate::ipc::protocol::Note { id: "note-1".into(), pitch: 60, start: 0.0, length: 1.0, velocity: 0.8 },
        });
        assert_eq!(s.pattern.notes().len(), 1);

        let id = s.pattern.notes()[0].id().to_string();
        dispatch(&mut s, Command::DeleteNote { request_id: "2".into(), id });
        assert_eq!(s.pattern.notes().len(), 0);
    }

    #[test]
    fn test_loop_toggle() {
        let mut s = state();
        dispatch(&mut s, Command::LoopToggle { request_id: "1".into(), enabled: true });
        assert!(s.loop_enabled);
        dispatch(&mut s, Command::LoopToggle { request_id: "2".into(), enabled: false });
        assert!(!s.loop_enabled);
    }
}
