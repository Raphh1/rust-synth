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
    // Paramètres ADSR en secondes / niveau
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,

    // Etat interne
    stage: EnvelopeStage,
    level: f64,       // amplitude courante [0.0, 1.0]
    prev_gate: f64,   // gate du sample précédent (pour détecter les fronts)
}

impl EnvelopeNode {
    pub fn new(attack: f64, decay: f64, sustain: f64, release: f64) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
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
    /// input[0] = gate (1.0 note on, 0.0 note off)
    /// output   = amplitude courante [0.0, 1.0]
    fn process(&mut self, input: &[f64], output: &mut f64, sample_rate: f64) {
        let gate = input.first().copied().unwrap_or(0.0);

        // Détection front montant → Attack
        if gate > 0.5 && self.prev_gate <= 0.5 {
            self.stage = EnvelopeStage::Attack;
        }
        // Détection front descendant → Release
        if gate <= 0.5 && self.prev_gate > 0.5 {
            self.stage = EnvelopeStage::Release;
        }
        self.prev_gate = gate;

        match self.stage {
            EnvelopeStage::Idle => {
                self.level = 0.0;
            }
            EnvelopeStage::Attack => {
                self.level += 1.0 / (self.attack * sample_rate);
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.stage = EnvelopeStage::Decay;
                }
            }
            EnvelopeStage::Decay => {
                self.level -= (1.0 - self.sustain) / (self.decay * sample_rate);
                if self.level <= self.sustain {
                    self.level = self.sustain;
                    self.stage = EnvelopeStage::Sustain;
                }
            }
            EnvelopeStage::Sustain => {
                self.level = self.sustain;
            }
            EnvelopeStage::Release => {
                self.level -= self.level / (self.release * sample_rate);
                if self.level < 0.0001 {
                    self.level = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            }
        }

        *output = self.level;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idle_without_gate() {
        let mut env = EnvelopeNode::new(0.01, 0.1, 0.7, 0.3);
        let mut out = 0.0;
        env.process(&[0.0], &mut out, 44100.0);
        assert_eq!(out, 0.0);
    }

    #[test]
    fn test_attack_rises() {
        let mut env = EnvelopeNode::new(0.01, 0.1, 0.7, 0.3);
        let sr = 44100.0;
        let mut out = 0.0;
        // Premier sample avec gate=1 → passage en Attack
        env.process(&[1.0], &mut out, sr);
        assert!(out > 0.0, "level should rise during attack");
    }

    #[test]
    fn test_reaches_sustain_after_attack_and_decay() {
        let mut env = EnvelopeNode::new(0.001, 0.001, 0.5, 0.3);
        let sr = 44100.0;
        let mut out = 0.0;
        // Laisser tourner assez longtemps pour finir Attack + Decay
        for _ in 0..500 {
            env.process(&[1.0], &mut out, sr);
        }
        assert!((out - 0.5).abs() < 0.01, "should be at sustain level");
    }

    #[test]
    fn test_release_goes_to_zero() {
        let mut env = EnvelopeNode::new(0.001, 0.001, 0.5, 0.001);
        let sr = 44100.0;
        let mut out = 0.0;
        // Monter jusqu'au sustain
        for _ in 0..500 { env.process(&[1.0], &mut out, sr); }
        // Couper le gate → Release
        for _ in 0..500 { env.process(&[0.0], &mut out, sr); }
        assert!(out < 0.01, "should be near zero after release");
    }
}
