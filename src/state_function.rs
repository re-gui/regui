

// TODO the reuse_with method

mod executor; pub use executor::*;
mod live_value; pub use live_value::*;

/// A "**stateful function**".
///
/// # Example
/// ```
/// use regui::StateFunction;
/// 
/// struct Adder {
///     value: i32,
/// }
/// 
/// impl StateFunction for Adder {
///     type Input = i32;
///     type Output = i32;
///     fn build(input: Self::Input) -> (Self::Output, Self) {
///         (input, Self { value: input })
///     }
///     fn changed(&mut self, input: Self::Input) -> Self::Output {
///         self.value += input;
///         self.value
///     }
/// }
/// 
/// let (value, mut adder) = Adder::build(42);
/// assert_eq!(value, 42);
/// assert_eq!(adder.changed(10), 52);
/// ```
pub trait StateFunction: 'static { // TODO remove 'static
    /// The input of the function.
    type Input;

    /// The output of the function.
    type Output;

    /// Produce the output and the function itself from the input.
    ///
    /// This function will be called on the first invocation of the function.
    #[must_use]
    fn build(input: Self::Input) -> (Self::Output, Self);

    /// Get an updated output from the input.
    ///
    /// This function will be called on every subsequent invocation of the function.
    #[must_use]
    fn changed(&mut self, input: Self::Input) -> Self::Output;

    /// Whether the function can be reused with the given props.
    ///
    /// TODO doc
    #[must_use]
    fn reuse_with(&self, _input: &Self::Input) -> bool {
        true
    }
}
