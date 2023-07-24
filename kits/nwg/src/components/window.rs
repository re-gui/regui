
use std::{rc::Rc, cell::RefCell, ops::Deref};

use native_windows_gui as nwg;
use regui::{StateFunction, component::{GetFromCache, FunctionsCache}};

use crate::{WithNwgControlHandle, NwgNode, WindowEvent};


impl WithNwgControlHandle for nwg::Window {
    fn nwg_control_handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
    fn position(&self) -> (i32, i32) {
        self.position()
    }
    fn size(&self) -> (u32, u32) {
        self.size()
    }
}

#[derive(Clone, PartialEq)]
pub enum WindowContent {
    None,
    Nodes(Vec<NwgNode<nwg::ControlHandle>>),
}

impl From<Vec<NwgNode<nwg::ControlHandle>>> for WindowContent {
    fn from(nodes: Vec<NwgNode<nwg::ControlHandle>>) -> Self {
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
    pub icon: Option<Rc<nwg::Icon>>,
    pub enabled: bool,
    pub content: WindowContent,
    pub on_window_event: Rc<dyn Fn(&WindowEvent)>,
    // TODO icon
    // TODO status bar icon
    // TODO status bar progress and notifications
    // TODO menu
    // TODO movable
    // TODO resizable
    // TODO borderless
    // TODO without title bar
    // TODO without button
    // TODO transparent
    // TODO topmost
    // TODO always on top
    // TODO always on bottom
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
            icon: None,
            enabled: true,
            content: WindowContent::None,
            on_window_event: Rc::new(|_| {}),
        }
    }
}

impl Window {
    pub fn builder() -> WindowBuilder {
        WindowBuilder {
            props: Window::default(),
        }
    }
}

impl GetFromCache for Window {
    type Out = nwg::ControlHandle;
    fn get(self, cache: &FunctionsCache) -> Self::Out {
        cache.eval::<WindowFunction>(self)
    }
}

pub struct WindowBuilder {
    props: Window,
}

impl WindowBuilder {
    pub fn id(mut self, id: i32) -> Self {
        self.props.id = Some(id);
        self
    }

    pub fn parent_handle(mut self, parent_handle: nwg::ControlHandle) -> Self {
        self.props.parent_handle = Some(parent_handle);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.props.title = title.into();
        self
    }

    pub fn initial_position(mut self, x: i32, y: i32) -> Self {
        self.props.initial_position = Some((x, y));
        self
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.props.position = Some((x, y));
        self
    }

    pub fn initial_size(mut self, width: u32, height: u32) -> Self {
        self.props.initial_size = Some((width, height));
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.props.size = Some((width, height));
        self
    }

    pub fn icon(mut self, icon: Rc<nwg::Icon>) -> Self {
        self.props.icon = Some(icon);
        self
    }

    pub fn icon_opt(mut self, icon: Option<Rc<nwg::Icon>>) -> Self {
        self.props.icon = icon;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.props.enabled = enabled;
        self
    }

    pub fn content(mut self, content: impl Into<WindowContent>) -> Self {
        self.props.content = content.into();
        self
    }

    pub fn on_window_event(mut self, on_window_event: impl Fn(&WindowEvent) + 'static) -> Self {
        self.props.on_window_event = Rc::new(on_window_event);
        self
    }

    pub fn build(self) -> Window {
        self.props
    }
}

pub struct WindowFunction {
    native: Rc<nwg::Window>,
    props: Option<Window>,
    on_window_event_ref: Rc<RefCell<Rc<dyn Fn(&WindowEvent)>>>,
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
                    let _ = node.borrow_mut().from_parent(&self.native.handle);
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

        builder = builder.icon(input.icon.as_ref().map(|icon| icon.deref()));

        builder
            .build(&mut window)
            .unwrap();

        window.set_enabled(input.enabled);

        let on_window_event_ref = Rc::new(RefCell::new(input.on_window_event.clone()));

        let window = Rc::new(window);

        let handler = nwg::full_bind_event_handler(&window.handle, {
            let on_window_event_ref = on_window_event_ref.clone();
            let window = Rc::downgrade(&window);
            move |event, _evt_data, handle| {
                let window = match window.upgrade() {
                    Some(window) => window,
                    None => return,
                };
                if handle != window.handle {
                    return;
                }
                let event = match event {
                    nwg::Event::OnWindowClose => {
                        // TODO after this event, the window will be hidden. This is not the correct
                        // behavior. The user should be able to decide if really close the window
                        Some(WindowEvent::CloseRequest)
                    }
                    nwg::Event::OnWindowMaximize => {
                        Some(WindowEvent::Maximize)
                    }
                    nwg::Event::OnWindowMinimize => {
                        Some(WindowEvent::Minimize)
                    }
                    nwg::Event::OnMove => {
                        let pos = window.position();
                        Some(WindowEvent::Moved(pos.0, pos.1))
                    }
                    nwg::Event::OnResize => {
                        let size = window.size();
                        Some(WindowEvent::Resized(size.0 as u32, size.1 as u32))
                    }
                    _ => None
                };
                if let Some(event) = event {
                    let on_window_event = on_window_event_ref.borrow().clone();
                    (on_window_event)(&event);
                }
            }
        });

        let content = input.content.clone();

        let mut s = Self {
            native: window,
            props: Some(input),
            on_window_event_ref,
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

        if !Rc::ptr_eq(&input.on_window_event, &self.props().on_window_event) {
            self.on_window_event_ref.replace(input.on_window_event.clone());
        }

        if input.icon != self.props().icon {
            self.native.set_icon(input.icon.as_ref().map(|icon| icon.deref()));
        }

        if input.enabled != self.props().enabled {
            self.native.set_enabled(input.enabled);
        }

        self.props = Some(input);
        self.native.handle.clone()
    }

    fn reuse_with(&self, input: &Self::Input) -> bool {
        input.parent_handle == self.props().parent_handle && input.id == self.props().id
    }
}
