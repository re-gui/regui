use std::ops::Deref;

use iui::UI;
use iui::controls::Window as IuiWindow;
use iui::prelude::WindowType;
use regui::decl_function_component;
use regui::function_component::{Cx, ComponentFunction};

use crate::Control;


pub struct WindowProps {
    ui: UI,
    title: String,
    child: Option<Control>
}

impl WindowProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            title: "".into(),
            child: None,
        }
    }
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }
    pub fn child(mut self, child: Control) -> Self {
        self.child = Some(child);
        self
    }
    pub fn get(self, cx: &mut Cx) {
        Window::eval(cx, self)
    }
}

decl_function_component!(pub Window window(WindowProps) -> ());

impl Window {
    pub fn builder(ui: &UI) -> WindowProps {
        WindowProps::new(ui)
    }
}

fn window(props: &WindowProps, cx: &mut Cx) -> () {
    let win = cx.use_state(|| {
        let mut win = IuiWindow::new(&props.ui, &props.title, 200, 200, WindowType::NoMenubar);
        if let Some(child) = &props.child {
            win.set_child(&props.ui, child.control.deref().clone());
        }
        win.show(&props.ui);

        win
    });

    let old_child = cx.use_state(|| props.child.clone());

    let mut win = win.get();

    win.set_title(&props.ui, &props.title);
    if let Some(child) = &props.child {
        if let Some(old_child) = old_child.get() {
            if old_child != child.clone() {
                win.set_child(&props.ui, child.control.deref().clone());
                // TODO
            }
        } else {
            win.set_child(&props.ui, child.control.deref().clone());
        }
    }

    old_child.set(props.child.clone());
}