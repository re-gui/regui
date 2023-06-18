

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


pub trait AsLiveValue<T> {
    fn into_live_value(self) -> LiveValue<T>;
}

impl<T> AsLiveValue<T> for T {
    fn into_live_value(self) -> LiveValue<T> {
        LiveValue {
            value: self,
            emitter: LiveValueEmitter {
                inner: Rc::new(RefCell::new(LiveInner {
                    listener: None,
                })),
            },
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

//impl<T: Clone> Live<T> for T {
//    fn current_value(&self) -> T {
//        self.clone()
//    }
//    fn listen(&self, _listener: impl FnMut() + 'static) {}
//}