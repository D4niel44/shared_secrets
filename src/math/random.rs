use rug::rand::RandState;

/// Provides a random number generator that can
/// be use to generate ModIntegers.
pub struct Rng<'a> {
    inner: RandState<'a>,
}

impl<'a> Rng<'a> {
    /// Creates a new random number generator.
    pub fn new() -> Self {
        Rng {
            inner: RandState::new(),
        }
    }

    // util method to retrieve the internal wrapped instance of RandState
    pub(crate) fn internal_rep(&mut self) -> &mut RandState<'a> {
        &mut self.inner
    }
}
