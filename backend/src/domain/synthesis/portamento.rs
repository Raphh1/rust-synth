/// Portamento : glisse la frquence courante vers la frquence cible.
///
/// Principe : interpolation exponentielle entre freq_current et freq_target
/// sur une dure `time` secondes. Quand time=0  pas de glisse (instantan).
#[derive(Debug)]
pub struct Portamento {
    time: f64,          // dure du glisse en secondes (0 = off)
    current_freq: f64,  // frquence courante (Hz)
    target_freq: f64,   // frquence cible (Hz)
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

    /// Dclenche le glisse vers une nouvelle note.
    pub fn set_target(&mut self, freq: f64) {
        self.target_freq = freq;
        // Si portamento off  pas de glisse, on saute directement
        if self.time < 0.001 {
            self.current_freq = freq;
        }
    }

    /// Avance d'un sample et retourne la frquence courante.
    pub fn tick(&mut self, sample_rate: f64) -> f64 {
        if self.time < 0.001 || (self.current_freq - self.target_freq).abs() < 0.01 {
            self.current_freq = self.target_freq;
            return self.current_freq;
        }

        // Coefficient de lissage exponentiel : plus time est grand, plus c'est lent
        // On vise ~99% atteint aprs `time` secondes  coeff par sample
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
        // Premier sample : on est encore proche de 440
        let f = p.tick(44100.0);
        assert!(f < 441.0, "should still be near 440 Hz, got {}", f);
    }

    #[test]
    fn test_portamento_reaches_target() {
        let sr = 44100.0;
        let mut p = Portamento::new(0.1, 440.0);
        p.set_target(880.0);
        // Laisser tourner 1 seconde (bien plus que 0.1s de portamento)
        let mut f = 0.0;
        for _ in 0..(sr as usize) {
            f = p.tick(sr);
        }
        assert!((f - 880.0).abs() < 1.0, "should reach 880 Hz, got {}", f);
    }

    #[test]
    fn test_portamento_goes_down() {
        let sr = 44100.0;
        let mut p = Portamento::new(0.1, 880.0);
        p.set_target(440.0);
        let mut f = 880.0;
        for _ in 0..(sr as usize) {
            f = p.tick(sr);
        }
        assert!((f - 440.0).abs() < 1.0, "should reach 440 Hz, got {}", f);
    }
}
