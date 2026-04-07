use crate::domain::synthesis::dsp::DspNode;

// Biquad low-pass filter (RBJ Audio EQ Cookbook).
#[derive(Debug)]
pub struct FilterNode {
    cutoff: f64,    // Hz [20, 20000]
    resonance: f64, // [0.0, 1.0] mapped to Q [0.5, 20.0]

    b0: f64, b1: f64, b2: f64,
    a1: f64, a2: f64,

    x1: f64, x2: f64,
    y1: f64, y2: f64,
}

impl FilterNode {
    pub fn new(cutoff: f64, resonance: f64, sample_rate: f64) -> Self {
        let mut node = Self {
            cutoff,
            resonance,
            b0: 0.0, b1: 0.0, b2: 0.0,
            a1: 0.0, a2: 0.0,
            x1: 0.0, x2: 0.0,
            y1: 0.0, y2: 0.0,
        };
        node.recompute(sample_rate);
        node
    }

    pub fn set_cutoff(&mut self, cutoff: f64, sample_rate: f64) {
        self.cutoff = cutoff.clamp(20.0, 20000.0);
        self.recompute(sample_rate);
    }

    pub fn set_resonance(&mut self, resonance: f64, sample_rate: f64) {
        self.resonance = resonance.clamp(0.0, 1.0);
        self.recompute(sample_rate);
    }

    fn recompute(&mut self, sample_rate: f64) {
        let cutoff = self.cutoff.clamp(20.0, sample_rate / 2.0 - 1.0);
        let q = 0.5 + self.resonance * 19.5;

        let w0 = 2.0 * std::f64::consts::PI * cutoff / sample_rate;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * q);

        let b0 = (1.0 - cos_w0) / 2.0;
        let b1 = 1.0 - cos_w0;
        let b2 = (1.0 - cos_w0) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }
}

impl DspNode for FilterNode {
    fn process(&mut self, input: &[f64], output: &mut f64, _sample_rate: f64) {
        let x0 = input.first().copied().unwrap_or(0.0);

        let y0 = self.b0 * x0
            + self.b1 * self.x1
            + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = x0;
        self.y2 = self.y1;
        self.y1 = y0;

        *output = y0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silence_in_silence_out() {
        let mut f = FilterNode::new(1000.0, 0.5, 44100.0);
        let mut out = 0.0;
        f.process(&[0.0], &mut out, 44100.0);
        assert_eq!(out, 0.0);
    }

    #[test]
    fn test_low_cutoff_attenuates_high_freq() {
        let sr = 44100.0;
        let mut f = FilterNode::new(200.0, 0.0, sr);
        let freq = 10000.0;
        let mut out = 0.0;
        let mut sum_sq = 0.0;

        for i in 0..4096 {
            let x = (2.0 * std::f64::consts::PI * freq * i as f64 / sr).sin();
            f.process(&[x], &mut out, sr);
            if i > 2048 { sum_sq += out * out; }
        }
        let rms = (sum_sq / 2048.0).sqrt();
        assert!(rms < 0.1, "high freq should be attenuated, rms={}", rms);
    }

    #[test]
    fn test_open_cutoff_passes_signal() {
        let sr = 44100.0;
        let mut f = FilterNode::new(18000.0, 0.0, sr);
        let freq = 440.0;
        let mut out = 0.0;
        let mut sum_sq = 0.0;

        for i in 0..4096 {
            let x = (2.0 * std::f64::consts::PI * freq * i as f64 / sr).sin();
            f.process(&[x], &mut out, sr);
            if i > 2048 { sum_sq += out * out; }
        }
        let rms = (sum_sq / 2048.0).sqrt();
        assert!(rms > 0.5, "low freq should pass through, rms={}", rms);
    }
}
