use std::ops::Deref;

use iui::UI;
use regui::{decl_function_component, function_component::{Cx, ComponentFunction}};

use crate::Control;



pub struct LabelProps {
    pub ui: UI,
    pub text: String,
}

impl LabelProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            text: "".into(),
        }
    }
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.into();
        self
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        Label::eval(cx, self)
    }
}

decl_function_component!(pub Label label(LabelProps) -> Control);

impl Label {
    pub fn builder(ui: &UI) -> LabelProps {
        LabelProps::new(ui)
    }
}

fn label(props: &LabelProps, cx: &mut Cx) -> Control {
    let label = cx.use_state(|| iui::controls::Label::new(&props.ui, &props.text));
    let control = cx.use_ref(|| Control::new(label.get()));

    let mut label = label.get();

    label.set_text(&props.ui, &props.text);

    control.deref().clone()
}