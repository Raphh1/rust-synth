use crate::domain::synthesis::dsp::{DspNode, OscillatorNode};
use crate::domain::synthesis::envelope::EnvelopeNode;
use crate::domain::synthesis::filter::FilterNode;
use crate::domain::synthesis::lfo::{Lfo, LfoTarget};
use crate::domain::synthesis::portamento::Portamento;
use crate::ipc::dispatcher::EngineState;

pub fn export_wav(state: &EngineState, path: &str) -> Result<(), String> {
    let sr = state.sample_rate;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sr as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)
        .map_err(|e| format!("Cannot create WAV file: {}", e))?;
    let bpm = state.transport.bpm();
    let notes = state.pattern.notes().to_vec();

    let pattern_len = notes.iter()
        .map(|n| n.start() + n.length())
        .fold(4.0_f64, f64::max);

    let total_samples = (pattern_len / bpm * 60.0 * sr).ceil() as usize;

    let mut osc        = OscillatorNode::new(state.wavetable.clone(), 440.0, 0.8);
    let mut envelope   = EnvelopeNode::new(state.envelope_attack, state.envelope_decay, state.envelope_sustain, state.envelope_release);
    let mut filter     = FilterNode::new(state.filter_cutoff, state.filter_resonance, sr);
    let mut lfo        = Lfo::new(state.lfo_rate, state.lfo_depth, state.lfo_shape.clone(), state.lfo_target.clone());
    let mut portamento = Portamento::new(0.0, 440.0);

    let beats_per_sample = bpm / 60.0 / sr;
    let mut seq_pos = 0.0_f64;

    let midi_to_hz = |pitch: u8| -> f64 {
        440.0 * 2_f64.powf((pitch as f64 - 69.0) / 12.0)
    };

    for _ in 0..total_samples {
        // Séquenceur : note active à seq_pos
        let mut gate = 0.0_f64;
        for note in &notes {
            if note.start() <= seq_pos && seq_pos < note.start() + note.length() {
                portamento.set_target(midi_to_hz(note.pitch()));
                gate = 1.0;
                break;
            }
        }

        // LFO
        let lfo_val        = lfo.tick(sr);
        let lfo_cutoff_mod = if state.lfo_target == LfoTarget::Cutoff { lfo_val * 2000.0 } else { 0.0 };
        let lfo_pitch_mod  = if state.lfo_target == LfoTarget::Pitch  { lfo_val }           else { 0.0 };
        let lfo_vol_mod    = if state.lfo_target == LfoTarget::Volume { lfo_val }            else { 0.0 };

        // Oscillateur
        let freq = portamento.tick(sr) * 2_f64.powf(lfo_pitch_mod / 12.0);
        osc.set_frequency(freq);
        let mut osc_out = 0.0;
        osc.process(&[], &mut osc_out, sr);

        // Enveloppe
        let mut env_out = 0.0;
        envelope.process(&[gate], &mut env_out, sr);

        // Filtre
        let modulated_cutoff = (state.filter_cutoff + lfo_cutoff_mod).clamp(20.0, 20000.0);
        filter.set_cutoff(modulated_cutoff, sr);
        let mut flt_out = 0.0;
        filter.process(&[osc_out * env_out], &mut flt_out, sr);

        // Sortie
        let volume     = (1.0 + lfo_vol_mod).clamp(0.0, 1.5);
        let sample     = (flt_out * volume).clamp(-1.0, 1.0);
        let sample_i16 = (sample * i16::MAX as f64) as i16;

        writer.write_sample(sample_i16)
            .map_err(|e| format!("Write error: {}", e))?;

        seq_pos += beats_per_sample;
    }

    writer.finalize().map_err(|e| format!("Finalize error: {}", e))?;
    Ok(())
}
