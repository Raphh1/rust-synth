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
