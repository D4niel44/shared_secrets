use rug::rand::RandState;

/// Provides a random number generator that can
/// be use to generate ModIntegers.
pub struct Rng<'a> {
    inner: RandState<'a>,
}

impl<'a> Rng<'a> {
    /// Creates a new random number generator.
    ///
    /// # Returns
    ///
    /// A new number generator.
    pub fn new() -> Self {
        Rng {
            inner: RandState::new(),
        }
    }

    /// Utility method to retrieve the internal wrapped instance of RandState
    ///
    /// # Returns
    ///
    /// An internal wrapped intance of RandState.
    pub(super) fn internal_rep(&mut self) -> &mut RandState<'a> {
        &mut self.inner
    }
}
