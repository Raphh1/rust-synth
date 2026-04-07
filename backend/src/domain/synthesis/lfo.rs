/// Forme d'onde du LFO
#[derive(Debug, Clone, PartialEq)]
pub enum LfoShape {
    Sine,
    Square,
    Triangle,
    Saw,
}

impl LfoShape {
    /// Depuis la valeur numérique envoyée par l'IPC (lfo.shape)
    /// 0=sine, 1=square, 2=triangle, 3=saw
    pub fn from_f64(v: f64) -> Self {
        match v as u8 {
            1 => LfoShape::Square,
            2 => LfoShape::Triangle,
            3 => LfoShape::Saw,
            _ => LfoShape::Sine,
        }
    }
}

/// Cible de modulation du LFO
#[derive(Debug, Clone, PartialEq)]
pub enum LfoTarget {
    Cutoff,
    Pitch,
    Volume,
}

impl LfoTarget {
    /// 0=cutoff, 1=pitch, 2=volume
    pub fn from_f64(v: f64) -> Self {
        match v as u8 {
            1 => LfoTarget::Pitch,
            2 => LfoTarget::Volume,
            _ => LfoTarget::Cutoff,
        }
    }
}

/// LFO (Low Frequency Oscillator).
/// Génère une valeur de modulation [-1.0, 1.0] à chaque tick.
/// Ne fait PAS partie du DspGraph — il est consulté par l'engine
/// pour moduler les paramètres (cutoff, pitch, volume).
#[derive(Debug)]
pub struct Lfo {
    rate: f64,    // Hz
    depth: f64,   // [0.0, 1.0]
    shape: LfoShape,
    pub target: LfoTarget,
    phase: f64,   // [0.0, 1.0)
}

impl Lfo {
    pub fn new(rate: f64, depth: f64, shape: LfoShape, target: LfoTarget) -> Self {
        Self { rate, depth, shape, target, phase: 0.0 }
    }

    pub fn set_rate(&mut self, rate: f64)   { self.rate  = rate.max(0.01); }
    pub fn set_depth(&mut self, depth: f64) { self.depth = depth.clamp(0.0, 1.0); }
    pub fn set_shape(&mut self, shape: LfoShape)   { self.shape  = shape; }
    pub fn set_target(&mut self, target: LfoTarget) { self.target = target; }

    /// Avance d'un sample et retourne la valeur de modulation [-depth, +depth].
    pub fn tick(&mut self, sample_rate: f64) -> f64 {
        let value = match self.shape {
            LfoShape::Sine => {
                (2.0 * std::f64::consts::PI * self.phase).sin()
            }
            LfoShape::Square => {
                if self.phase < 0.5 { 1.0 } else { -1.0 }
            }
            LfoShape::Triangle => {
                if self.phase < 0.5 {
                    4.0 * self.phase - 1.0
                } else {
                    3.0 - 4.0 * self.phase
                }
            }
            LfoShape::Saw => {
                2.0 * self.phase - 1.0
            }
        };

        self.phase += self.rate / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        value * self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_starts_at_zero() {
        let mut lfo = Lfo::new(1.0, 1.0, LfoShape::Sine, LfoTarget::Cutoff);
        let v = lfo.tick(44100.0);
        assert!(v.abs() < 0.001, "sine LFO should start near 0");
    }

    #[test]
    fn test_depth_zero_outputs_zero() {
        let mut lfo = Lfo::new(5.0, 0.0, LfoShape::Sine, LfoTarget::Cutoff);
        for _ in 0..100 {
            assert_eq!(lfo.tick(44100.0), 0.0);
        }
    }

    #[test]
    fn test_output_within_depth_range() {
        let depth = 0.7;
        let mut lfo = Lfo::new(2.0, depth, LfoShape::Sine, LfoTarget::Cutoff);
        for _ in 0..44100 {
            let v = lfo.tick(44100.0);
            assert!(v >= -depth - 1e-9 && v <= depth + 1e-9,
                "value {} out of range [-{}, {}]", v, depth, depth);
        }
    }

    #[test]
    fn test_square_only_two_values() {
        let mut lfo = Lfo::new(1.0, 1.0, LfoShape::Square, LfoTarget::Pitch);
        for _ in 0..44100 {
            let v = lfo.tick(44100.0);
            assert!(v == 1.0 || v == -1.0, "square should be ±1, got {}", v);
        }
    }

    #[test]
    fn test_shape_from_f64() {
        assert_eq!(LfoShape::from_f64(0.0), LfoShape::Sine);
        assert_eq!(LfoShape::from_f64(1.0), LfoShape::Square);
        assert_eq!(LfoShape::from_f64(2.0), LfoShape::Triangle);
        assert_eq!(LfoShape::from_f64(3.0), LfoShape::Saw);
    }

    #[test]
    fn test_target_from_f64() {
        assert_eq!(LfoTarget::from_f64(0.0), LfoTarget::Cutoff);
        assert_eq!(LfoTarget::from_f64(1.0), LfoTarget::Pitch);
        assert_eq!(LfoTarget::from_f64(2.0), LfoTarget::Volume);
    }
}
