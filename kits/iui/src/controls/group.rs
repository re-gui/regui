use std::{ops::Deref, cell::RefCell};

use iui::UI;
use regui::{decl_function_component, function_component::{Cx, ComponentFunction}};

use crate::Control;



pub struct GroupProps {
    pub ui: UI,
    pub title: String,
    pub margined: bool,
    pub child: Option<Control>,
}

impl GroupProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            title: "".into(),
            margined: false,
            child: None,
        }
    }
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }
    pub fn margined(mut self, margined: bool) -> Self {
        self.margined = margined;
        self
    }
    pub fn child(mut self, child: Control) -> Self {
        self.child = Some(child);
        self
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        Group::eval(cx, self)
    }
}

decl_function_component!(pub Group group(GroupProps) -> Control);

impl Group {
    pub fn builder(ui: &UI) -> GroupProps {
        GroupProps::new(ui)
    }
}

fn group(props: &GroupProps, cx: &mut Cx) -> Control {
    let group = cx.use_state(|| iui::controls::Group::new(&props.ui, &props.title));
    let control = cx.use_ref(|| Control::new(group.get()));

    let mut group = group.get();

    group.set_title(&props.ui, &props.title);

    group.set_margined(&props.ui, props.margined);

    let old_child = cx.use_ref(|| RefCell::new(Option::<Control>::None));
    if let Some(child) = &props.child {
        let old_child = old_child.borrow();
        if let Some(old_child) = old_child.deref() {
            if old_child.clone() != child.clone() {
                group.set_child(&props.ui, child.control.deref().clone());
                // TODO
            }
        } else {
            group.set_child(&props.ui, child.control.deref().clone());
        }
    }
    *old_child.deref().borrow_mut() = props.child.clone();

    control.deref().clone()
}
