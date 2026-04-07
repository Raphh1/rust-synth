use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::domain::synthesis::dsp::{DspNode, OscillatorNode};
use crate::domain::synthesis::envelope::EnvelopeNode;
use crate::domain::synthesis::filter::FilterNode;
use crate::domain::synthesis::lfo::{Lfo, LfoShape, LfoTarget};
use crate::domain::synthesis::portamento::Portamento;
use crate::domain::synthesis::wavetable::Wavetable;
use crate::ipc::dispatcher::{dispatch, EngineState};
use crate::ipc::protocol::{Command, ErrorCode, Response};


pub fn run() -> io::Result<()> {
    let state = Arc::new(Mutex::new(EngineState::new()));

    let audio_state = Arc::clone(&state);
    thread::spawn(move || {
        if let Err(e) = run_audio(audio_state) {
            eprintln!("[audio] error: {}", e);
        }
    });

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        eprintln!("[backend input] {}", line);
        let response = match serde_json::from_str::<Command>(&line) {
            Ok(cmd) => {
                let mut s = state.lock().unwrap();
                dispatch(&mut s, cmd)
            }
            Err(e) => {
                eprintln!("[backend deserialization error] {}", e);
                Response::Error {
                    request_id: "unknown".into(),
                    code: ErrorCode::InvalidMessage,
                    message: format!("Invalid JSON: {}", e),
                }
            }
        };
        let response_str = serde_json::to_string(&response).unwrap();
        eprintln!("[backend output] {}", response_str);
        writeln!(stdout, "{}", &response_str)?;
        stdout.flush()?;
    }

    Ok(())
}

fn run_audio(state: Arc<Mutex<EngineState>>) -> Result<(), Box<dyn std::error::Error>> {
    let host   = cpal::default_host();
    let device = host.default_output_device().ok_or("no output device")?;
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f64;
    let channels    = config.channels() as usize;

    // Stocker le sample rate réel dans l'état pour l'export
    {
        let mut s = state.lock().unwrap();
        s.sample_rate = sample_rate;
    }

    let mut osc        = OscillatorNode::new(Wavetable::sine(Wavetable::DEFAULT_SIZE), 440.0, 0.8);
    let mut envelope   = EnvelopeNode::new(0.01, 0.1, 0.7, 0.3);
    let mut filter     = FilterNode::new(1000.0, 0.5, sample_rate);
    let mut lfo        = Lfo::new(1.0, 0.5, LfoShape::Sine, LfoTarget::Cutoff);
    let mut portamento = Portamento::new(0.0, 440.0);
    let mut seq_pos      = 0.0_f64;
    let mut was_playing  = false; // pour détecter le front montant Play

    let midi_to_hz = |pitch: u8| -> f64 {
        440.0 * 2_f64.powf((pitch as f64 - 69.0) / 12.0)
    };

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _| {
            let snap = {
                let s = state.lock().unwrap();
                Snapshot {
                    is_playing:     s.transport.is_playing(),
                    loop_enabled:   s.loop_enabled,
                    bpm:            s.transport.bpm(),
                    wavetable:      s.wavetable.clone(),
                    env_a:          s.envelope_attack,
                    env_d:          s.envelope_decay,
                    env_s:          s.envelope_sustain,
                    env_r:          s.envelope_release,
                    flt_cutoff:     s.filter_cutoff,
                    flt_res:        s.filter_resonance,
                    lfo_rate:       s.lfo_rate,
                    lfo_depth:      s.lfo_depth,
                    lfo_shape:      s.lfo_shape.clone(),
                    lfo_target:     s.lfo_target.clone(),
                    port_time:      s.portamento_time,
                    notes:          s.pattern.notes().to_vec(),
                }
            };

            // Mettre  jour les params DSP
            envelope.set_attack(snap.env_a);
            envelope.set_decay(snap.env_d);
            envelope.set_sustain(snap.env_s);
            envelope.set_release(snap.env_r);
            filter.set_cutoff(snap.flt_cutoff, sample_rate);
            filter.set_resonance(snap.flt_res, sample_rate);
            lfo.set_rate(snap.lfo_rate);
            lfo.set_depth(snap.lfo_depth);
            lfo.set_shape(snap.lfo_shape);
            lfo.set_target(snap.lfo_target.clone());
            portamento.set_time(snap.port_time);
            osc.set_wavetable(snap.wavetable);

            // Reset seq_pos au début de chaque Play
            if snap.is_playing && !was_playing {
                seq_pos = 0.0;
            }
            was_playing = snap.is_playing;

            let beats_per_sample = snap.bpm / 60.0 / sample_rate;
            let pattern_len = snap.notes.iter()
                .map(|n| n.start() + n.length())
                .fold(4.0_f64, f64::max);

            for frame in data.chunks_mut(channels) {
                if !snap.is_playing {
                    for out in frame.iter_mut() { *out = 0.0; }
                    continue;
                }

                // Séquenceur : trouver la note active
                let mut gate = 0.0_f64;
                for note in &snap.notes {
                    if note.start() <= seq_pos && seq_pos < note.start() + note.length() {
                        portamento.set_target(midi_to_hz(note.pitch()));
                        gate = 1.0;
                        break; // monophonique
                    }
                }

                // LFO
                let lfo_val = lfo.tick(sample_rate);
                let lfo_cutoff_mod = if snap.lfo_target == LfoTarget::Cutoff { lfo_val * 2000.0 } else { 0.0 };
                let lfo_pitch_mod  = if snap.lfo_target == LfoTarget::Pitch  { lfo_val } else { 0.0 };
                let lfo_vol_mod    = if snap.lfo_target == LfoTarget::Volume  { lfo_val } else { 0.0 };

                // Fréquence + portamento + modulation pitch LFO
                let freq = portamento.tick(sample_rate) * 2_f64.powf(lfo_pitch_mod / 12.0);
                osc.set_frequency(freq);

                // Oscillateur → envelope → filtre → sortie
                let mut osc_out = 0.0;
                osc.process(&[], &mut osc_out, sample_rate);

                let mut env_out = 0.0;
                envelope.process(&[gate], &mut env_out, sample_rate);

                let modulated_cutoff = (snap.flt_cutoff + lfo_cutoff_mod).clamp(20.0, 20000.0);
                filter.set_cutoff(modulated_cutoff, sample_rate);

                let mut flt_out = 0.0;
                filter.process(&[osc_out * env_out], &mut flt_out, sample_rate);

                let volume = (1.0 + lfo_vol_mod).clamp(0.0, 1.5);
                let sample_val = (flt_out * volume) as f32;

                // Écrire le même sample sur tous les canaux (mono → stéréo)
                for out in frame.iter_mut() { *out = sample_val; }

                // Avancer le séquenceur une seule fois par frame
                seq_pos += beats_per_sample;
                if seq_pos >= pattern_len {
                    seq_pos = if snap.loop_enabled { seq_pos - pattern_len } else { pattern_len };
                }
            }
        },
        |err| eprintln!("[audio] stream error: {}", err),
        None,
    )?;

    stream.play()?;
    loop { thread::sleep(std::time::Duration::from_secs(3600)); }
}

/// Snapshot de l'EngineState pour viter de tenir le Mutex pendant le rendu audio.
struct Snapshot {
    is_playing:   bool,
    loop_enabled: bool,
    bpm:          f64,
    wavetable:    Wavetable,
    env_a:        f64,
    env_d:        f64,
    env_s:        f64,
    env_r:        f64,
    flt_cutoff:   f64,
    flt_res:      f64,
    lfo_rate:     f64,
    lfo_depth:    f64,
    lfo_shape:    LfoShape,
    lfo_target:   LfoTarget,
    port_time:    f64,
    notes:        Vec<crate::domain::sequencing::pattern::NoteEvent>,
}
