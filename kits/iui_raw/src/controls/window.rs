use std::{cell::RefCell, rc::Rc, ffi::c_void};

use regui::function_component::{Cx, ComponentFunction};

use crate::{Control, basic_control_methods, control_with_child_methods};

#[derive(Clone)]
pub struct Window {
    w: *mut libui_ffi::uiWindow,
    control: Control,
    data: Rc<RefCell<WindowData>>
}

struct WindowData {
    child: Option<Control>,
    on_close: Box<Box<dyn Fn() -> bool>>
}

pub struct WindowProps {
    pub enabled: bool,
    pub show: bool,
    pub title: String,
    pub initial_size: (i32, i32),
    pub child: Option<Control>,
    pub on_close: Box<dyn Fn() -> bool>,
}

impl From<Window> for Control {
    fn from(w: Window) -> Self {
        w.control
    }
}

basic_control_methods!(Window, WindowProps);
control_with_child_methods!(Window, WindowProps);

impl Window {
    /// Create a new window.
    pub fn new(
        title: String,
        initial_size: (i32, i32),
    ) -> Self {
        let w = unsafe {
            let title = std::ffi::CString::new(title).unwrap();
            libui_ffi::uiNewWindow(
                title.as_ptr(),
                initial_size.0,
                initial_size.1,
                0 // TODO
            )
        };
        let basic_control = Control::new_raw(w as *mut libui_ffi::uiControl);

        Self {
            w,
            control: basic_control,
            data: Rc::new(RefCell::new(WindowData {
                child: None,
                on_close: Box::new(Box::new(|| true)),
            }))
        }
    }

    /// Creates the builder for a new window.
    pub fn functional() -> WindowProps {
        WindowProps {
            enabled: true,
            show: true,
            title: "".into(),
            initial_size: (200, 200),
            child: None,
            on_close: Box::new(|| false),
        }
    }

    /// What to do when the window is closed.
    ///
    /// If true is returned, the window will be hidden.
    pub fn on_close(&mut self, on_close: Box<dyn Fn() -> bool>) {
        extern "C" fn c_callback(_w: *mut libui_ffi::uiWindow, data: *mut c_void) -> i32 {
            let on_close = unsafe {
                let on_close = data as *const Box<dyn Fn() -> bool>;
                &*on_close
            };
            on_close() as i32
        }

        let mut data = self.data.borrow_mut();
        data.on_close = Box::new(on_close);

        let on_close_ptr = &*data.on_close as *const Box<dyn Fn() -> bool> as *mut c_void;
        unsafe {
            libui_ffi::uiWindowOnClosing(self.w, Some(c_callback), on_close_ptr);
        }
    }
}

impl WindowProps {
    /// Get the window.
    pub fn eval(self, cx: &mut Cx) -> Window {
        Self::call(self, cx)
    }
}

impl ComponentFunction for WindowProps {
    type Props = Self;
    type Out = Window;
    fn call<'a>(props: Self::Props, cx: &mut Cx) -> Self::Out {
        let w = cx.use_ref(|| RefCell::new(Window::new(
            props.title.clone(),
            props.initial_size,
        )));
        let mut w = w.borrow_mut();

        w.enable(props.enabled);
        w.show(props.show);

        if let Some(child) = &props.child {
            w.set_child(child.clone());
        } else {
            w.remove_child();
        }

        w.on_close(props.on_close);

        w.clone()
    }
}