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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::synthesis::dsp::DspNode;

    const SR: f64 = 44100.0;

    fn run(env: &mut EnvelopeNode, gate: f64, samples: usize) -> f64 {
        let mut out = 0.0;
        for _ in 0..samples { env.process(&[gate], &mut out, SR); }
        out
    }

    #[test]
    fn starts_idle_at_zero() {
        let mut env = EnvelopeNode::new(0.01, 0.1, 0.7, 0.3);
        let mut out = 0.0;
        env.process(&[0.0], &mut out, SR);
        assert_eq!(out, 0.0);
    }

    #[test]
    fn attack_rises_to_one() {
        let attack = 0.01;
        let mut env = EnvelopeNode::new(attack, 0.001, 1.0, 0.001);
        // Run enough samples for full attack
        let samples = (attack * SR * 1.5) as usize;
        let out = run(&mut env, 1.0, samples);
        assert!((out - 1.0).abs() < 0.01, "expected ~1.0, got {}", out);
    }

    #[test]
    fn sustain_level_reached_after_decay() {
        let sustain = 0.5;
        let mut env = EnvelopeNode::new(0.001, 0.01, sustain, 0.3);
        // Run past attack + decay
        let samples = (0.1 * SR) as usize;
        let out = run(&mut env, 1.0, samples);
        assert!((out - sustain).abs() < 0.01, "expected sustain ~{}, got {}", sustain, out);
    }

    #[test]
    fn release_returns_to_zero() {
        let mut env = EnvelopeNode::new(0.001, 0.001, 0.7, 0.01);
        // Trigger attack/decay/sustain
        run(&mut env, 1.0, (0.1 * SR) as usize);
        // Now release
        let out = run(&mut env, 0.0, (0.5 * SR) as usize);
        assert!(out < 0.001, "expected ~0 after release, got {}", out);
    }

    #[test]
    fn retrigger_restarts_attack() {
        let mut env = EnvelopeNode::new(0.001, 0.001, 0.7, 0.3);
        run(&mut env, 1.0, (0.1 * SR) as usize); // reach sustain
        run(&mut env, 0.0, 10);                   // start release
        // Retrigger
        let mut out = 0.0;
        env.process(&[1.0], &mut out, SR);
        // Should be back in attack stage (level rising)
        let out2 = run(&mut env, 1.0, 5);
        assert!(out2 > 0.0);
    }
}
