use std::{rc::Rc, fmt::{Debug, Formatter}};



pub struct Callback<In = (), Out = ()> {
    callback: Rc<dyn Fn(In) -> Out>,
}

impl Debug for Callback<(), ()> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback") // TODO args and return type
    }
}

impl<In, Out> Clone for Callback<In, Out> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback.clone(),
        }
    }
}

impl<In, Out> Callback<In, Out> {
    pub fn call(&self, input: In) -> Out {
        (self.callback)(input)
    }
}

impl<Out> Callback<(), Out> {
    pub fn call_no_args(&self) -> Out {
        (self.callback)(())
    }
    pub fn no_args(func: impl Fn() -> Out + 'static) -> Self {
        Self {
            callback: Rc::new(move |_| func()),
        }
    }
}

// TODO maybe into instead of from?
impl<In, Out, F: Fn(In) -> Out + 'static> From<F> for Callback<In, Out> { // TODO static necessary?
    fn from(func: F) -> Self {
        Self {
            callback: Rc::new(func),
        }
    }
}

impl<In, Out> PartialEq for Callback<In, Out> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.callback, &other.callback)
    }
}