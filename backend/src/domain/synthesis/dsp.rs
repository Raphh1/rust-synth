use std::collections::HashMap;

use crate::domain::synthesis::wavetable::Wavetable;

pub trait DspNode: Send {
    fn process(&mut self, input: &[f64], output: &mut f64, sample_rate: f64);
}

#[derive(Debug)]
pub struct OscillatorNode {
    wavetable: Wavetable,
    phase: f64, 
    frequency: f64,
    gain: f64,
}

#[derive(Debug)]
pub struct OutputNode {
    gain: f64,
}

impl OscillatorNode {
    pub fn new(wavetable: Wavetable, frequency: f64, gain: f64) -> Self {
        Self {
            wavetable,
            phase: 0.0,
            frequency,
            gain,
        }
    }

    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    pub fn set_gain(&mut self, gain: f64) {
        self.gain = gain;
    }

    pub fn set_wavetable(&mut self, wavetable: Wavetable) {
        self.wavetable = wavetable;
    }
}

impl OutputNode {
    pub fn new(gain: f64) -> Self {
        Self { gain }
    }

    pub fn set_gain(&mut self, gain: f64) {
        self.gain = gain;
    }
}

impl DspNode for OscillatorNode {
    fn process(&mut self, _input: &[f64], output: &mut f64, sample_rate: f64) {
        let sample = self.wavetable.sample_at(self.phase);
        self.phase += self.frequency / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        *output = sample * self.gain;
    }
}

impl DspNode for OutputNode {
    fn process(&mut self, input: &[f64], output: &mut f64, _sample_rate: f64) {
        let sum: f64 = input.iter().sum();
        *output += sum * self.gain;
    }
}

pub struct DspGraphNode {
    pub id: String,
    pub node: Box<dyn DspNode>,
    pub input_ids: Vec<String>,
}

impl DspGraphNode {
    pub fn new(id: String, node: Box<dyn DspNode>, input_ids: Vec<String>) -> Self {
        Self { id, node, input_ids }
    }
}

pub struct DspGraph {
    nodes: Vec<DspGraphNode>,  // ordonns topologiquement par PatchCompiler
    buffers: HashMap<String, f64>,  // buffer de sortie par nud
}

impl DspGraph {
    pub fn new(nodes: Vec<DspGraphNode>) -> Self {
        Self {
            nodes,
            buffers: HashMap::new(),
        }
    }

    pub fn tick(&mut self, sample_rate: f64) -> f64 {
        for node in &mut self.nodes {
            let input_values: Vec<f64> = node.input_ids.iter()
                .map(|id| *self.buffers.get(id).unwrap_or(&0.0))
                .collect();
            let output_buffer = self.buffers.entry(node.id.clone()).or_insert(0.0);
            node.node.process(&input_values, output_buffer, sample_rate);
        }
        *self.buffers.get("output").unwrap_or(&0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::synthesis::wavetable::Wavetable;

    const SR: f64 = 44100.0;

    #[test]
    fn oscillator_output_bounded() {
        let mut osc = OscillatorNode::new(Wavetable::sine(2048), 440.0, 1.0);
        for _ in 0..SR as usize {
            let mut out = 0.0;
            osc.process(&[], &mut out, SR);
            assert!(out.abs() <= 1.0, "oscillator out of range: {}", out);
        }
    }

    #[test]
    fn oscillator_gain_zero_outputs_silence() {
        let mut osc = OscillatorNode::new(Wavetable::sine(2048), 440.0, 0.0);
        let mut out = 0.0;
        osc.process(&[], &mut out, SR);
        assert_eq!(out, 0.0);
    }

    #[test]
    fn oscillator_phase_wraps() {
        let mut osc = OscillatorNode::new(Wavetable::sine(2048), SR, 1.0); // 1 cycle/sample
        let mut out = 0.0;
        // After exactly SR samples the phase should have wrapped SR times — just check it doesn't panic
        for _ in 0..1000 {
            osc.process(&[], &mut out, SR);
        }
    }

    #[test]
    fn output_node_sums_inputs_with_gain() {
        let mut node = OutputNode::new(2.0);
        let mut out = 0.0;
        node.process(&[1.0, 1.5], &mut out, SR);
        assert!((out - 5.0).abs() < 1e-9, "expected 5.0, got {}", out);
    }
}
