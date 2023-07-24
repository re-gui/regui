use std::{cell::RefCell, rc::{Rc, Weak}};

use repaint::{nalgebra::Vector2, BasicPainter, Color, Canvas};
use winit::{event::WindowEvent, event_loop::ControlFlow, window::Window as WinitWindow};

use crate::SPainter;

use super::{BasicSkiaWindow, ReLoop, ReWindow};

struct DataInner {
    on_event: Option<Rc<dyn Fn(&WindowEvent, &mut ControlFlow)>>,
    on_paint: Option<Rc<dyn Fn(&mut SPainter, Vector2<f64>)>>,
    to_repaint: bool,
}

impl Default for DataInner {
    fn default() -> Self {
        Self {
            on_event: None,
            on_paint: None,
            to_repaint: false,
        }
    }
}

#[derive(Clone)]
pub struct DataLink {
    inner: Weak<RefCell<DataInner>>,
}

impl DataLink {
    pub fn on_event(&mut self, on_event: impl Fn(&WindowEvent, &mut ControlFlow) + 'static) {
        if let Some(inner) = self.inner.upgrade() {
            inner.borrow_mut().on_event = Some(Rc::new(on_event));
        }
    }

    pub fn on_paint(&mut self, on_paint: impl Fn(&mut SPainter, Vector2<f64>) + 'static) {
        if let Some(inner) = self.inner.upgrade() {
            inner.borrow_mut().on_paint = Some(Rc::new(on_paint));
        }
    }

    pub fn request_redraw(&mut self) {
        if let Some(inner) = self.inner.upgrade() {
            inner.borrow_mut().to_repaint = true;
        }
    }
}

pub struct SkiaWindow {
    skia_window: BasicSkiaWindow,
    data: Rc<RefCell<DataInner>>,
}

impl SkiaWindow {
    pub fn new_no_register(re_loop: &mut ReLoop) -> Self {
        Self {
            skia_window: BasicSkiaWindow::new_no_register(re_loop),
            data: Rc::new(RefCell::new(DataInner::default())),
        }
    }

    pub fn new(re_loop: &mut ReLoop) -> Rc<RefCell<Self>> {
        let s = Self::new_no_register(re_loop);
        re_loop.register_window(s)
    }

    pub fn data_link(&self) -> DataLink {
        DataLink {
            inner: Rc::downgrade(&self.data),
        }
    }
}

impl ReWindow for SkiaWindow {
    fn instance(&self) -> &WinitWindow {
        self.skia_window.instance()
    }

    fn handle_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        self.skia_window.handle_event(event, control_flow);
        //match event {
        //    WindowEvent::CloseRequested => {
        //        return Some(ControlFlow::Exit);
        //    }
        //    _ => {}
        //}
        let on_event = self.data.borrow().on_event.clone();
        if let Some(on_event) = &on_event {
            on_event(event, control_flow);
        }
    }

    fn main_events_cleared(&mut self, control_flow: &mut ControlFlow) {
        self.skia_window.main_events_cleared(control_flow);
        let to_repaint = self.data.borrow().to_repaint;
    }

    fn draw(&mut self, control_flow: &mut ControlFlow) {
        self.skia_window.draw(control_flow);
        let on_paint = self.data.borrow().on_paint.clone();
        if let Some(on_paint) = &on_paint {
            self.skia_window.paint_with_skia_painter(|painter| {
                let size = painter.canvas().shape();
                let size = Vector2::new(size.width() as f64, size.height() as f64);
                on_paint(painter, size);
            });
        } else {
            self.skia_window.paint_with_skia_painter(|painter| {
                painter.clear(Color::WHITE.into());
                painter.line(Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0), Color::RED.into());
                let n = 50;
                for i in 0..n {
                    let p = i as f64 / n as f64 * 500.0;
                    let p2 = (n - i) as f64 / n as f64 * 500.0;
                    painter.line(Vector2::new(p, 0.0), Vector2::new(0.0, p2), Color::RED.into());
                }
            });
        }
        //self.request_redraw();
    }
}