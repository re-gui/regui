use std::{rc::{Rc, Weak}, cell::RefCell, collections::HashMap};

use winit::{event_loop::{EventLoop, ControlFlow}, window::{Window as WinitWindow, WindowId}, event::{Event, WindowEvent}, platform::run_return::EventLoopExtRunReturn};

mod skia; pub use skia::*;
mod basic_window; pub use basic_window::*;
mod widget_window; pub use widget_window::*;

pub struct ReLoop {
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, Weak<RefCell<dyn ReWindow>>>,
}

impl ReLoop {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            windows: HashMap::new(),
        }
    }

    #[must_use]
    pub fn run(&mut self) -> i32 {
        self.event_loop.run_return(|event, _target, control_flow| {
            control_flow.set_wait();

            // remove dropped windows
            self.windows.retain(|_, v| v.upgrade().is_some());

            //#[allow(deprecated)]
            match event {
                Event::WindowEvent { event, window_id } => {
                    let window = self.windows.get(&window_id);
                    if let Some(window) = window {
                        if let Some(window) = window.upgrade() {
                            window.borrow_mut().handle_event(&event, control_flow);
                        }
                    };
                },
                Event::RedrawRequested(window_id) => {
                    let window = self.windows.get(&window_id);
                    if let Some(window) = window {
                        if let Some(window) = window.upgrade() {
                            window.borrow_mut().draw(control_flow);
                        }
                    };
                }
                Event::RedrawEventsCleared => {
                    for (_id, window) in self.windows.iter() {
                        if let Some(window) = window.upgrade() {
                            window.borrow_mut().main_events_cleared(control_flow);
                        }
                    }
                }
                Event::LoopDestroyed => {}
                _ => (),
            }
        })
    }

    pub fn register_window<W: ReWindow>(&mut self, window: W) -> Rc<RefCell<W>> {
        let window_id = window.instance().id();
        let window = Rc::new(RefCell::new(window));
        let dyn_window: Rc<RefCell<dyn ReWindow>> = window.clone();
        self.windows.insert(window_id, Rc::downgrade(&dyn_window));
        window
    }
}

pub fn windowing_loop() -> i32 {

    let mut re_loop = ReLoop::new();

    let _skia_window = BasicSkiaWindow::new(&mut re_loop);

    re_loop.run()
}

pub trait ReWindow: 'static {
    fn instance(&self) -> &WinitWindow;
    fn handle_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow);
    fn main_events_cleared(&mut self, control_flow: &mut ControlFlow);
    fn draw(&mut self, control_flow: &mut ControlFlow);
    fn size(&self) -> (u32, u32) {
        let size = self.instance().inner_size();
        (size.width, size.height)
    }
}

/*pub trait ReWindowUtils: 'static {
    fn on_instance_return<R>(&self, f: impl FnOnce(&WinitWindow) -> R) -> R;
}

impl<W: ReWindow> ReWindowUtils for W {
    fn on_instance_return<R>(&self, f: impl FnOnce(&WinitWindow) -> R) -> R {
        let mut r = None;
        self.on_instance(&|w| {
            r = Some(f(w));
        });
        r.unwrap()
    }
}*/

