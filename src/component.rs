

// TODO the reuse_with method

use std::{rc::{Rc, Weak}, cell::RefCell};

/// A "**stateful function**".
pub trait Component: 'static { // TODO remove 'static
    type Props;
    //type Input;
    type Output;
    #[must_use]
    fn build(props: Self::Props) -> (Self::Output, Self);
    #[must_use]
    fn changed(&mut self, props: Self::Props) -> Self::Output;
    fn reuse_with(&self, props: &Self::Props) -> bool {
        true
    }
}

// A live value is a value that can be updated at any time.
// To represent a live value, we use a `Live` type:
struct LiveInner<T> {
    getter: Box<dyn Fn() -> T>,
    listener: Option<RefCell<Box<dyn FnMut()>>>,
}

pub struct LiveValue<T> {
    inner: Rc<RefCell<LiveInner<T>>>,
}

impl<T> LiveValue<T> {
    pub fn current_value(&self) -> T {
        (self.inner.borrow().getter)()
    }
    pub fn listen(&self, listener: impl FnMut() + 'static) { // TODO maybe mut?
        self.inner.borrow_mut().listener = Some(RefCell::new(Box::new(listener)));
    }
}

pub struct LiveLink<T> {
    inner: Rc<RefCell<LiveInner<T>>>,
}

impl<T> LiveLink<T> {
    pub fn new(getter: impl Fn() -> T + 'static) -> Self {
        Self {
            inner: Rc::new(RefCell::new(LiveInner {
                getter: Box::new(getter),
                listener: None,
            })),
        }
    }
    pub fn set(&self, getter: impl Fn() -> T + 'static) { // TODO maybe mut?
        self.inner.borrow_mut().getter = Box::new(getter);
        self.inner.borrow().listener.as_ref().map(|listener| listener.borrow_mut()());
    }
    pub fn live_value(&self) -> LiveValue<T> {
        LiveValue {
            inner: self.inner.clone(),
        }
    }
}

/// Declares a univoque relationship <code>[Component::Props] -> [Component]</code>.
///
/// When a type implements this trait, it means that it has a corresponding [`Component`] that can be built from its props.
pub trait ComponentProps: Sized {
    type AssociatedComponent: Component<Props = Self>;
    fn build(self) -> (<Self::AssociatedComponent as Component>::Output, Self::AssociatedComponent) {
        Self::AssociatedComponent::build(self)
    }
}

pub trait Live<T> {
    fn current_value(&self) -> T;
    fn listen(&self, listener: impl FnMut() + 'static);
}

impl<T: Clone> Live<T> for T {
    fn current_value(&self) -> T {
        self.clone()
    }
    fn listen(&self, _listener: impl FnMut() + 'static) {}
}

impl<T> Live<T> for LiveValue<T> {
    fn current_value(&self) -> T {
        self.current_value()
    }
    fn listen(&self, listener: impl FnMut() + 'static) {
        self.listen(listener)
    }
}