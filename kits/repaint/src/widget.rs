use std::{rc::Rc, any::Any, cell::RefCell};

use repaint::nalgebra::Vector2;
use repaint_with_skia_safe::SkiaPainter;
use taffy::tree::NodeId;

pub type SPainter<'canvas> = SkiaPainter<'canvas, 'canvas>;

pub enum Event {
    MouseEnter,
    MouseLeave,
}

pub trait Widget: 'static {
    fn box_layout_leaf(&self) -> NodeId;
    fn children(&self) -> &[Rc<RefCell<dyn Widget>>];
    fn paint<'canvas>(
        &mut self,
        painter: &mut SPainter<'canvas>,
        size: Vector2<f64>,
        resources: Option<Box<dyn Any>>
    ) -> Option<Box<dyn Any>>;
}