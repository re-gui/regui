

// TODO the reuse_with method

use std::{rc::Rc, cell::RefCell};

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

// A live value is a value that can be updated at any time.
// To represent a live value, we use a `Live` type:
struct LiveInner {
    listener: Option<RefCell<Rc<RefCell<dyn FnMut()>>>>,
}

pub struct LiveValueEmitter {
    inner: Rc<RefCell<LiveInner>>,
}

impl LiveValueEmitter {
    pub fn listen(&self, listener: impl FnMut() + 'static) { // TODO maybe mut?
        self.inner.borrow_mut().listener = Some(RefCell::new(Rc::new(RefCell::new(listener))));
    }
}

pub struct LiveValue<T> {
    pub value: T,
    pub emitter: LiveValueEmitter,
}

impl<T> LiveValue<T> {
    pub fn into_tuple(self) -> (T, LiveValueEmitter) {
        (self.value, self.emitter)
    }
}

#[derive(Clone)]
pub struct LiveLink {
    inner: Rc<RefCell<LiveInner>>,
}

impl LiveLink {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(LiveInner {
                listener: None,
            })),
        }
    }
    pub fn tell_update(&self) {
        let listener = if let Some(listener) = self.inner.borrow().listener.as_ref() {
            //(listener.borrow().borrow_mut())();
            Some(listener.borrow().clone())
        } else {
            None
        };
        if let Some(listener) = listener {
            (listener.borrow_mut())();
        }
        //self.inner.borrow().listener.as_ref().map(|listener| listener.borrow_mut()());
    }
    pub fn make_live_value<T>(&self, value: T) -> LiveValue<T> {
        LiveValue {
            value,
            emitter: LiveValueEmitter {
                inner: self.inner.clone(),
            },
        }
    }
}




/// Declares a univoque relationship <code>[StatefulFunction::Props] -> [StatefulFunction]</code>.
///
/// When a type implements this trait, it means that it has a corresponding [`StatefulFunction`] that can be built from its props.
pub trait StateFunctionProps: Sized {
    type AssociatedFunction: StateFunction<Input = Self>;
    fn build(self) -> (<Self::AssociatedFunction as StateFunction>::Output, Self::AssociatedFunction) {
        Self::AssociatedFunction::build(self)
    }
}

//impl<T: Clone> Live<T> for T {
//    fn current_value(&self) -> T {
//        self.clone()
//    }
//    fn listen(&self, _listener: impl FnMut() + 'static) {}
//}