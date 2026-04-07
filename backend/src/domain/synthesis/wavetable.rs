#[derive(Debug, Clone)]
pub struct Wavetable {
    samples: Vec<f64>,
}

impl Wavetable {

    pub const DEFAULT_SIZE: usize = 2048;

    fn new(samples: Vec<f64>) -> Self {
        Self { samples }
    }

    pub fn samples(&self) -> &[f64] {
        &self.samples
    }

    pub fn sine(size: usize) -> Self {
        let samples = (0..size)
            .map(|i| (i as f64 / size as f64 * 2.0 * std::f64::consts::PI).sin())
            .collect();
        Self::new(samples)
    }

    pub fn saw(size: usize) -> Self {
        let samples = (0..size)
            .map(|i| 2.0 * (i as f64 / size as f64) - 1.0)
            .collect();
        Self::new(samples)
    }

    pub fn square(size: usize) -> Self {
        let samples = (0..size)
            .map(|i| if i < size / 2 { 1.0 } else { -1.0 })
            .collect();
        Self::new(samples)
    }

    /// Construit une Wavetable depuis un vecteur de samples envoy par l'IPC.
    pub fn from_samples(samples: Vec<f64>) -> Self {
        Self::new(samples)
    }

    pub fn sample_at(&self, phase: f64) -> f64 {
        let len = self.samples.len();

        // Convert normalized phase [0.0, 1.0) to a floating-point index into the table
        let exact_index = phase * len as f64;

        // The two neighbouring samples that bracket the exact index
        let left_index = exact_index as usize % len;
        let right_index = (left_index + 1) % len;

        // Fractional distance between the two neighbours (0.0 = at left, 1.0 = at right)
        let fraction = exact_index - exact_index.floor();

        // Linear interpolation between the two neighbours
        self.samples[left_index] * (1.0 - fraction) + self.samples[right_index] * fraction
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_starts_at_zero_and_has_correct_size() {
        let wt = Wavetable::sine(64);
        assert_eq!(wt.samples().len(), 64);
        assert!(wt.samples()[0].abs() < 1e-9);
    }

    #[test]
    fn sample_at_midpoint_sine_near_zero() {
        let wt = Wavetable::sine(2048);
        // phase 0.5 → sin(π) ≈ 0
        let v = wt.sample_at(0.5);
        assert!(v.abs() < 1e-3, "expected ~0 at half period, got {}", v);
    }

    #[test]
    fn sample_at_quarter_sine_near_one() {
        let wt = Wavetable::sine(2048);
        // phase 0.25 → sin(π/2) ≈ 1
        let v = wt.sample_at(0.25);
        assert!((v - 1.0).abs() < 1e-2, "expected ~1 at quarter period, got {}", v);
    }

    #[test]
    fn from_samples_preserves_values() {
        let samples = vec![0.0, 0.5, 1.0, 0.5, 0.0, -0.5, -1.0, -0.5];
        let wt = Wavetable::from_samples(samples.clone());
        assert_eq!(wt.samples(), samples.as_slice());
    }

    #[test]
    fn square_first_half_positive_second_half_negative() {
        let wt = Wavetable::square(8);
        assert!(wt.samples()[..4].iter().all(|&v| v == 1.0));
        assert!(wt.samples()[4..].iter().all(|&v| v == -1.0));
    }

    #[test]
    fn saw_runs_from_minus_one_to_near_one() {
        let wt = Wavetable::saw(100);
        let s = wt.samples();
        assert!((s[0] - (-1.0)).abs() < 0.05);
        assert!((s[99] - 1.0).abs() < 0.05);
    }
}
