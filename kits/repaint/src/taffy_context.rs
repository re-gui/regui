use std::{cell::RefCell, rc::Rc, collections::VecDeque};

use taffy::{Taffy, style::{Style as TaffyStyle, AvailableSpace}, tree::{NodeId, Layout}, prelude::Size};

use crate::windowing::WidgetWindow;



struct TaffyContextInner {
    taffy: RefCell<Taffy>,
    to_update: RefCell<bool>,
    on_window_queue: RefCell<VecDeque<Box<dyn FnOnce(&mut WidgetWindow)>>>,
}

impl TaffyContextInner {
    fn new() -> Self {
        Self {
            taffy: RefCell::new(Taffy::new()),
            to_update: RefCell::new(false),
            on_window_queue: RefCell::new(VecDeque::new()),
        }
    }
}

#[derive(Clone)]
pub struct TaffyContext {
    inner: Rc<TaffyContextInner>,
}

impl TaffyContext {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(TaffyContextInner::new()),
        }
    }
    pub fn on_taffy<F: FnOnce(&Taffy) -> O, O>(&self, f: F) -> O {
        f(&mut self.inner.taffy.borrow())
    }
    pub fn on_taffy_mut<F: FnOnce(&mut Taffy) -> O, O>(&self, f: F) -> O {
        self.request_layout_update();
        f(&mut self.inner.taffy.borrow_mut())
    }

    fn request_layout_update(&self) {
        self.inner.to_update.replace(true);
        self.request_repaint();
    }
    pub fn update_requested(&self) -> bool {
        *self.inner.to_update.borrow()
    }
    /*pub fn do_update<F: FnOnce(&mut bool) -> O, O>(&self, f: F) -> O {
        self.inner.to_update.replace(true);
        f(&mut self.inner.to_update.borrow_mut())
    }*/

    pub fn request_repaint(&self) {
        self.on_window_mut(|window| window.request_repaint());
    }

    pub fn on_window_mut(&self, f: impl FnOnce(&mut WidgetWindow) + 'static) {
        self.inner.on_window_queue.borrow_mut().push_back(Box::new(f));
    }

    pub fn on_window(&self, f: impl FnOnce(&WidgetWindow) + 'static) {
        self.inner.on_window_queue.borrow_mut().push_back(Box::new(move |window| f(window)));
    }

    pub fn poll_on_window(&self) -> Option<Box<dyn FnOnce(&mut WidgetWindow)>> {
        self.inner.on_window_queue.borrow_mut().pop_front()
    }

    pub fn compute_layout(&self, root: NodeId, available_space: Size<AvailableSpace>) {
        self.inner.taffy.borrow_mut().compute_layout(root, available_space).unwrap();
        self.inner.to_update.replace(false);
    }

    pub fn layout(&self, root: NodeId) -> Layout {
        self.inner.taffy.borrow().layout(root).unwrap().clone()
    }
}