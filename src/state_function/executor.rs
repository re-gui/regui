
use super::*;

/// A simple manager for [`StateFunction`]s.
///
/// This sctruct can hold a [`StateFunction`] and will call [`StateFunction::build`] on the first invocation of [`Executor::eval`],
/// and [`StateFunction::changed`] on every subsequent invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Executor<F: StateFunction> {
    function: Option<F>,
}

impl<F: StateFunction> Executor<F> {
    /// Create a new [`Executor`] with no function state.
    ///
    /// Use [`Executor::from`] to create an [`Executor`] with a function state.
    pub fn new() -> Self {
        Self {
            function: None,
        }
    }

    /// Reset the function state.
    pub fn reset(&mut self) {
        self.function = None;
    }

    /// Evaluate the function with the given input.
    pub fn eval(&mut self, input: F::Input) -> F::Output {
        if let Some(function) = self.function.as_mut() {
            if function.reuse_with(&input) {
                return function.changed(input);
            }
        }
        let (output, function) = F::build(input);
        self.function = Some(function);
        output
    }
}

impl<F: StateFunction> Default for Executor<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: StateFunction> From<F> for Executor<F> {
    fn from(function: F) -> Self {
        Self {
            function: Some(function),
        }
    }
}