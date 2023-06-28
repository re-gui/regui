use std::{cell::RefCell, rc::{Rc}, borrow::BorrowMut};

use taffy::{Taffy, style::{Style as TaffyStyle, AvailableSpace}, tree::{NodeId, Layout}, prelude::Size};



struct TaffyContextInner {
    taffy: RefCell<Taffy>,
    to_update: RefCell<bool>,
    on_repaint_request: Box<dyn Fn()>,
}

impl TaffyContextInner {
    fn new(on_repaint_request: impl Fn() + 'static) -> Self {
        Self {
            taffy: RefCell::new(Taffy::new()),
            to_update: RefCell::new(false),
            on_repaint_request: Box::new(on_repaint_request),
        }
    }
}

#[derive(Clone)]
pub struct TaffyContext {
    inner: Rc<TaffyContextInner>,
}

impl TaffyContext {
    pub fn new(on_repaint_request: impl Fn() + 'static) -> Self {
        Self {
            inner: Rc::new(TaffyContextInner::new(on_repaint_request)),
        }
    }
    fn on_taffy<F: FnOnce(&Taffy) -> O, O>(&self, f: F) -> O {
        f(&mut self.inner.taffy.borrow())
    }
    fn on_taffy_mut<F: FnOnce(&mut Taffy) -> O, O>(&self, f: F) -> O {
        self.request_update();
        f(&mut self.inner.taffy.borrow_mut())
    }
    
    pub fn new_leaf(&self, style: TaffyStyle) -> NodeId {
        self.on_taffy_mut(|taffy| taffy.new_leaf(style)).unwrap()
    }

    pub fn set_style(&self, leaf: NodeId, style: TaffyStyle) {
        self.on_taffy_mut(|taffy| taffy.set_style(leaf, style)).unwrap();
    }

    pub fn get_style(&self, leaf: NodeId) -> TaffyStyle {
        self.on_taffy(|taffy| taffy.style(leaf).unwrap().clone())
    }

    pub fn get_layout(&self, leaf: NodeId) -> Layout {
        self.on_taffy(|taffy| taffy.layout(leaf).unwrap().clone())
    }

    fn request_update(&self) {
        self.inner.to_update.replace(true);
        self.request_repaint();
    }
    pub fn request_repaint(&self) {
        (self.inner.on_repaint_request)();
    }
    pub fn update_requested(&self) -> bool {
        *self.inner.to_update.borrow()
    }
    /*pub fn do_update<F: FnOnce(&mut bool) -> O, O>(&self, f: F) -> O {
        self.inner.to_update.replace(true);
        f(&mut self.inner.to_update.borrow_mut())
    }*/

    pub fn compute_layout(&self, root: NodeId, available_space: Size<AvailableSpace>) {
        self.inner.taffy.borrow_mut().compute_layout(root, available_space).unwrap();
        self.inner.to_update.replace(false);
    }

    pub fn layout(&self, root: NodeId) -> Layout {
        self.inner.taffy.borrow().layout(root).unwrap().clone()
    }
}