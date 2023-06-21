
use std::{rc::Rc, cell::RefCell};

/// A live value represents a value that can be updated at any time.
///
/// The value hold by this struct is not updated, but it provides
/// a [`LiveValueEmitter`] that can be used to listen for updates.
///
/// # Example
/// ```
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// use regui::*;
/// 
/// // create a new live link, this will be used to update the value
/// let link = LiveLink::new();
/// 
/// let changed: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
/// 
/// let (value, emitter) = link.make_live_value(42).into_tuple();
/// emitter.listen({
///     let changed = changed.clone();
///     move || { changed.replace(true); }
/// });
/// 
/// assert_eq!(value, 42);
/// assert!(!*changed.borrow());
///
/// link.tell_update();
/// assert!(*changed.borrow());
/// ```
pub struct LiveValue<T> {
    pub value: T,
    pub emitter: LiveValueEmitter,
}

impl<T> LiveValue<T> {
    /// Convert this live value into a tuple of the value and the emitter.
    ///
    /// Use this function to access the value and the emitter at the same time.
    pub fn into_tuple(self) -> (T, LiveValueEmitter) {
        (self.value, self.emitter)
    }
}

/// A live link is used to create live values.
#[derive(Clone)]
pub struct LiveLink {
    inner: Rc<RefCell<LiveInner>>,
}

pub struct LiveValueEmitter {
    inner: Rc<RefCell<LiveInner>>,
}

impl LiveValueEmitter {
    /// Listen for updates.
    pub fn listen(&self, listener: impl FnMut() + 'static) { // TODO maybe mut?
        self.inner.borrow_mut().listener = Some(RefCell::new(Rc::new(RefCell::new(listener))));
    }
}

impl LiveLink {
    /// Create a new live link.
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(LiveInner {
                listener: None,
            })),
        }
    }

    /// Tell the listener that the value has been updated.
    pub fn tell_update(&self) {
        let listener = self.inner.borrow()
            .listener
            .as_ref().map(|listener| listener.borrow().clone());

        if let Some(listener) = listener {
            (listener.borrow_mut())();
        }
    }

    /// Create a new live value from this live link.
    pub fn make_live_value<T>(&self, value: T) -> LiveValue<T> {
        LiveValue {
            value,
            emitter: LiveValueEmitter {
                inner: self.inner.clone(),
            },
        }
    }
}

struct LiveInner {
    listener: Option<RefCell<Rc<RefCell<dyn FnMut()>>>>, // TODO maybe remove the first cell and use Cell instead of RefCell for the second one
}