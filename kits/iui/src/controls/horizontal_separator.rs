use std::ops::Deref;

use iui::UI;
use regui::{decl_function_component, function_component::{Cx, ComponentFunction}};

use crate::Control;

pub struct HorizontalSeparatorProps {
    ui: UI,
}

impl HorizontalSeparatorProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
        }
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        HorizontalSeparator::eval(cx, self)
    }
}

decl_function_component!(pub HorizontalSeparator horizontal_separator(HorizontalSeparatorProps) -> Control);

impl HorizontalSeparator {
    pub fn builder(ui: &UI) -> HorizontalSeparatorProps {
        HorizontalSeparatorProps::new(ui)
    }
}

fn horizontal_separator(props: &HorizontalSeparatorProps, cx: &mut Cx) -> Control {
    let separator = cx.use_state(|| iui::controls::HorizontalSeparator::new(&props.ui));
    let control = cx.use_ref(|| Control::new(separator.get()));

    control.deref().clone()
}