use std::{cell::RefCell, rc::Rc};

use repaint::{nalgebra::Vector2, BasicPainter, Color, Canvas};
use winit::{event::WindowEvent, event_loop::ControlFlow, window::Window as WinitWindow};

use crate::SPainter;

use super::{SkiaWindow, ReLoop, ReWindow};


pub struct BasicSkiaWindow {
    skia_window: SkiaWindow,
    on_event: Option<Box<dyn FnMut(&WindowEvent) -> Option<ControlFlow>>>,
    on_paint: Option<Box<dyn FnMut(&mut SPainter, Vector2<f64>)>>,
}

impl BasicSkiaWindow {
    pub fn new_no_register(re_loop: &mut ReLoop) -> Self {
        Self {
            skia_window: SkiaWindow::new_no_register(re_loop),
            on_event: None,
            on_paint: None,
        }
    }

    pub fn new(re_loop: &mut ReLoop) -> Rc<RefCell<Self>> {
        let s = Self::new_no_register(re_loop);
        re_loop.register_window(s)
    }

    pub fn request_redraw(&mut self) {
        self.skia_window.request_redraw();
    }

    pub fn on_event(&mut self, on_event: impl FnMut(&WindowEvent) -> Option<ControlFlow> + 'static) {
        self.on_event = Some(Box::new(on_event));
    }

    pub fn on_paint(&mut self, on_paint: impl FnMut(&mut SPainter, Vector2<f64>) + 'static) {
        self.on_paint = Some(Box::new(on_paint));
    }
}

impl ReWindow for BasicSkiaWindow {
    fn instance(&self) -> &WinitWindow {
        self.skia_window.instance()
    }

    fn handle_event(&mut self, event: &WindowEvent) -> Option<ControlFlow> {
        if let Some(r) = self.skia_window.handle_event(event) {
            return Some(r);
        }
        //match event {
        //    WindowEvent::CloseRequested => {
        //        return Some(ControlFlow::Exit);
        //    }
        //    _ => {}
        //}
        if let Some(on_event) = &mut self.on_event {
            on_event(event)
        } else {
            None
        }
    }

    fn main_events_cleared(&mut self) {
        self.skia_window.main_events_cleared();
    }

    fn draw(&mut self) {
        self.skia_window.draw();
        if let Some(on_paint) = &mut self.on_paint {
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