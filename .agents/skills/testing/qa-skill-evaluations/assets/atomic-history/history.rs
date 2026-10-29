//! Deliberately faulty, synthetic skill-evaluation fixture; not product code.

/// Bounded recent observations for the exercise's declared input domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct History {
    values: Vec<i32>,
}

/// The supplied sample is outside the exercise's inclusive range.
#[derive(Debug, PartialEq, Eq)]
pub struct OutOfRange;

impl History {
    /// Create an empty history.
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Accept one sample, preserving state on rejection under the task contract.
    pub fn push(&mut self, sample: i32) -> Result<(), OutOfRange> {
        if self.values.len() == 3 {
            self.values.remove(0);
        }
        if !(-100..=100).contains(&sample) {
            return Err(OutOfRange);
        }
        self.values.push(sample);
        Ok(())
    }

    /// Borrow all accepted samples in arrival order.
    pub fn samples(&self) -> &[i32] {
        &self.values
    }
}
