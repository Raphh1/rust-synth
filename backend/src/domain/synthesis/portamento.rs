// Exponential frequency glide from current to target frequency over `time` seconds.
// When time = 0, the frequency jumps instantly.
#[derive(Debug)]
pub struct Portamento {
    time: f64,
    current_freq: f64,
    target_freq: f64,
}

impl Portamento {
    pub fn new(time: f64, initial_freq: f64) -> Self {
        Self {
            time,
            current_freq: initial_freq,
            target_freq: initial_freq,
        }
    }

    pub fn set_time(&mut self, time: f64) {
        self.time = time.max(0.0);
    }

    pub fn set_target(&mut self, freq: f64) {
        self.target_freq = freq;
        if self.time < 0.001 {
            self.current_freq = freq;
        }
    }

    pub fn tick(&mut self, sample_rate: f64) -> f64 {
        if self.time < 0.001 || (self.current_freq - self.target_freq).abs() < 0.01 {
            self.current_freq = self.target_freq;
            return self.current_freq;
        }

        // Exponential smoothing: ~99% reached after `time` seconds
        let coeff = (-9.0_f64 / (self.time * sample_rate)).exp();
        self.current_freq = self.target_freq + (self.current_freq - self.target_freq) * coeff;
        self.current_freq
    }

    pub fn current_freq(&self) -> f64 {
        self.current_freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_portamento_jumps_instantly() {
        let mut p = Portamento::new(0.0, 440.0);
        p.set_target(880.0);
        let f = p.tick(44100.0);
        assert_eq!(f, 880.0);
    }

    #[test]
    fn test_portamento_starts_at_current_freq() {
        let mut p = Portamento::new(1.0, 440.0);
        p.set_target(880.0);
        let f = p.tick(44100.0);
        assert!(f < 441.0, "should still be near 440 Hz, got {}", f);
    }

    #[test]
    fn test_portamento_reaches_target() {
        let sr = 44100.0;
        let mut p = Portamento::new(0.1, 440.0);
        p.set_target(880.0);
        let mut f = 0.0;
        for _ in 0..(sr as usize) { f = p.tick(sr); }
        assert!((f - 880.0).abs() < 1.0, "should reach 880 Hz, got {}", f);
    }

    #[test]
    fn test_portamento_goes_down() {
        let sr = 44100.0;
        let mut p = Portamento::new(0.1, 880.0);
        p.set_target(440.0);
        let mut f = 880.0;
        for _ in 0..(sr as usize) { f = p.tick(sr); }
        assert!((f - 440.0).abs() < 1.0, "should reach 440 Hz, got {}", f);
    }
}
