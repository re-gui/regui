
use std::{rc::Rc, cell::RefCell};

use native_windows_gui as nwg;
use regui::{StateFunction, StateFunctionProps};

use crate::{WithNwgControlHandle, NwgControlNode};


impl WithNwgControlHandle for nwg::Window {
    fn nwg_control_handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
}

#[derive(Clone, PartialEq)]
pub enum WindowContent {
    None,
    Nodes(Vec<NwgControlNode>),
}

impl From<Vec<NwgControlNode>> for WindowContent {
    fn from(nodes: Vec<NwgControlNode>) -> Self {
        Self::Nodes(nodes)
    }
}

pub struct Window {
    pub id: Option<i32>,
    pub parent_handle: Option<nwg::ControlHandle>,
    pub title: String, // TODO cow
    pub initial_position: Option<(i32, i32)>,
    pub position: Option<(i32, i32)>,
    pub initial_size: Option<(u32, u32)>,
    pub size: Option<(u32, u32)>,
    pub content: WindowContent,
    pub on_close_request: Rc<dyn Fn()>,
    pub on_maximize: Rc<dyn Fn()>,
    pub on_minimize: Rc<dyn Fn()>,
    pub on_moved: Rc<dyn Fn(i32, i32)>,
    pub on_resized: Rc<dyn Fn(u32, u32)>,
    // TODO icon
    // TODO status bar icon
    // TODO status bar progress and notifications
    // TODO menu
}

impl Default for Window {
    fn default() -> Self {
        Self {
            id: None,
            parent_handle: None,
            title: "SOME WINDOW".into(), // TODO empty string
            initial_position: None,
            position: None,
            initial_size: None,
            size: None,
            content: WindowContent::None,
            on_close_request: Rc::new(|| {}),
            on_maximize: Rc::new(|| {}),
            on_minimize: Rc::new(|| {}),
            on_moved: Rc::new(|_, _| {}),
            on_resized: Rc::new(|_, _| {}),
        }
    }
}

impl StateFunctionProps for Window {
    type AssociatedFunction = WindowFunction;
}

struct Callbacks {
    on_close_request: Rc<dyn Fn()>,
    on_maximize: Rc<dyn Fn()>,
    on_minimize: Rc<dyn Fn()>,
    on_moved: Rc<dyn Fn(i32, i32)>,
    on_resized: Rc<dyn Fn(u32, u32)>,
}

pub struct WindowFunction {
    native: Rc<nwg::Window>,
    props: Option<Window>,
    callbacks: Rc<RefCell<Callbacks>>,
    handler: nwg::EventHandler,
}

impl Drop for WindowFunction {
    fn drop(&mut self) {
        // Note: props have to be dropped before native
        self.props.take();

        // Note: handler have to be dropped before native
        nwg::unbind_event_handler(&self.handler);
    }
}

impl WindowFunction {
    fn set_content(&mut self, content: &WindowContent) {
        match content {
            WindowContent::None => {
                // nothing to do
            }
            WindowContent::Nodes(nodes) => {
                for node in nodes {
                    let _ = node.borrow_mut().handle_from_parent(&self.native.handle);
                }
            }
        }
    }
    fn props(&self) -> &Window {
        self.props.as_ref().unwrap()
    }
}

impl StateFunction for WindowFunction {
    type Input = Window;
    type Output = nwg::ControlHandle;

    fn build(input: Self::Input) -> (Self::Output, Self) {
        let mut window = nwg::Window::default();

        let mut builder = nwg::Window::builder()
            .title(&input.title);

        if input.parent_handle.is_some() {
            builder = builder.parent(input.parent_handle);
        }

        if let Some((x, y)) = input.initial_position {
            builder = builder.position((x, y));
        }

        if let Some((x, y)) = input.position {
            builder = builder.position((x, y));
        }

        if let Some((x, y)) = input.initial_size {
            builder = builder.size((x as i32, y as i32));
        }

        if let Some((x, y)) = input.size {
            builder = builder.size((x as i32, y as i32));
        }

        builder
            .build(&mut window)
            .unwrap();

        //let on_close_request_ref = Rc::new(RefCell::new(input.on_close_request.clone()));
        //let on_close_maximize_ref = Rc::new(RefCell::new(input.on_close_maximize.clone()));
        //let on_close_minimize_ref = Rc::new(RefCell::new(input.on_close_minimize.clone()));
        let callbacks = Rc::new(RefCell::new(Callbacks {
            on_close_request: input.on_close_request.clone(),
            on_maximize: input.on_maximize.clone(),
            on_minimize: input.on_minimize.clone(),
            on_moved: input.on_moved.clone(),
            on_resized: input.on_resized.clone(),
        }));

        let window = Rc::new(window);

        let handler = nwg::full_bind_event_handler(&window.handle, {
            let callbacks = callbacks.clone();
            let window = Rc::downgrade(&window);
            move |event, _evt_data, handle| {
                let window = match window.upgrade() {
                    Some(window) => window,
                    None => return,
                };
                if handle != window.handle {
                    return;
                }
                match event {
                    nwg::Event::OnWindowClose => {
                        (callbacks.borrow().on_close_request)();
                        // TODO after this event, the window will be hidden. This is not the correct
                        // behavior. The user should be able to decide if really close the window
                    }
                    nwg::Event::OnWindowMaximize => {
                        (callbacks.borrow().on_maximize)();
                    }
                    nwg::Event::OnWindowMinimize => {
                        (callbacks.borrow().on_minimize)();
                    }
                    nwg::Event::OnMove => {
                        let pos = window.position();
                        (callbacks.borrow().on_moved)(pos.0, pos.1);
                    }
                    nwg::Event::OnResize => {
                        let size = window.size();
                        (callbacks.borrow().on_resized)(size.0 as u32, size.1 as u32);
                    }
                    _ => {}
                }
            }
        });

        let content = input.content.clone();

        let mut s = Self {
            native: window,
            props: Some(input),
            callbacks,
            handler,
        };

        s.set_content(&content);

        (
            s.native.handle,
            s,
        )
    }

    fn changed(&mut self, input: Self::Input) -> Self::Output {
        assert_eq!(
            input.parent_handle,
            self.props().parent_handle,
            "Changing parent of a window is not supported."
        );

        if input.title != self.props().title {
            self.native.set_text(&input.title);
        }

        if let Some(pos) = input.position {
            let current_pos = self.native.position();
            if pos != current_pos {
                self.native.set_position(pos.0, pos.1);
            }
        }

        if let Some(size) = input.size {
            let current_size = self.native.size();
            if size != current_size {
                self.native.set_size(size.0, size.1);
            }
        }

        if input.content != self.props().content {
            self.set_content(&input.content);
        }

        if !Rc::ptr_eq(&input.on_close_request, &self.props().on_close_request) {
            self.callbacks.borrow_mut().on_close_request = input.on_close_request.clone();
        }

        if !Rc::ptr_eq(&input.on_maximize, &self.props().on_maximize) {
            self.callbacks.borrow_mut().on_maximize = input.on_maximize.clone();
        }

        if !Rc::ptr_eq(&input.on_minimize, &self.props().on_minimize) {
            self.callbacks.borrow_mut().on_minimize = input.on_minimize.clone();
        }

        if !Rc::ptr_eq(&input.on_moved, &self.props().on_moved) {
            self.callbacks.borrow_mut().on_moved = input.on_moved.clone();
        }

        if !Rc::ptr_eq(&input.on_resized, &self.props().on_resized) {
            self.callbacks.borrow_mut().on_resized = input.on_resized.clone();
        }

        self.props = Some(input);
        self.native.handle.clone()
    }

    fn reuse_with(&self, input: &Self::Input) -> bool {
        input.parent_handle == self.props().parent_handle && input.id == self.props().id
    }
}
