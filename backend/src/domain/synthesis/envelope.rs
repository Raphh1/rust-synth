use crate::domain::synthesis::dsp::DspNode;

#[derive(Debug, Clone, PartialEq)]
enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Debug)]
pub struct EnvelopeNode {
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,
    stage: EnvelopeStage,
    level: f64,
    prev_gate: f64,
}

impl EnvelopeNode {
    pub fn new(attack: f64, decay: f64, sustain: f64, release: f64) -> Self {
        Self {
            attack, decay, sustain, release,
            stage: EnvelopeStage::Idle,
            level: 0.0,
            prev_gate: 0.0,
        }
    }

    pub fn set_attack(&mut self, v: f64)  { self.attack  = v.max(0.001); }
    pub fn set_decay(&mut self, v: f64)   { self.decay   = v.max(0.001); }
    pub fn set_sustain(&mut self, v: f64) { self.sustain = v.clamp(0.0, 1.0); }
    pub fn set_release(&mut self, v: f64) { self.release = v.max(0.001); }
}

impl DspNode for EnvelopeNode {
    fn process(&mut self, input: &[f64], output: &mut f64, sample_rate: f64) {
        let gate = input.first().copied().unwrap_or(0.0);

        if gate > 0.5 && self.prev_gate <= 0.5 { self.stage = EnvelopeStage::Attack; }
        if gate <= 0.5 && self.prev_gate > 0.5 { self.stage = EnvelopeStage::Release; }
        self.prev_gate = gate;

        match self.stage {
            EnvelopeStage::Idle => { self.level = 0.0; }
            EnvelopeStage::Attack => {
                self.level += 1.0 / (self.attack * sample_rate);
                if self.level >= 1.0 { self.level = 1.0; self.stage = EnvelopeStage::Decay; }
            }
            EnvelopeStage::Decay => {
                self.level -= (1.0 - self.sustain) / (self.decay * sample_rate);
                if self.level <= self.sustain { self.level = self.sustain; self.stage = EnvelopeStage::Sustain; }
            }
            EnvelopeStage::Sustain => { self.level = self.sustain; }
            EnvelopeStage::Release => {
                self.level -= self.level / (self.release * sample_rate);
                if self.level < 0.0001 { self.level = 0.0; self.stage = EnvelopeStage::Idle; }
            }
        }

        *output = self.level;
    }
}
