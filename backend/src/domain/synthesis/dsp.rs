use crate::domain::synthesis::wavetable::Wavetable;

pub trait DspNode: Send {
    fn process(&mut self, input: &[f64], output: &mut f64, sample_rate: f64);
}

#[derive(Debug)]
pub struct OscillatorNode {
    wavetable: Wavetable,
    phase: f64, // position actuelle dans le cycle [0.0, 1.0)
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

#[derive(Debug)]
pub struct DspNode {
    pub id: String,
    pub node: Box<dyn DspNode>,
    pub input_ids: Vec<String>,
}

impl DspNode {
    pub fn new(id: String, node: Box<dyn DspNode>, input_ids: Vec<String>) -> Self {
        Self { id, node, input_ids }
    }
}

#[derive(Debug)]
pub struct DspGraph {
    nodes: Vec<DspGraphNode>,   // ordonnés topologiquement par PatchCompiler
    buffers: HashMap<String, f64>,  // buffer de sortie par nœud
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
