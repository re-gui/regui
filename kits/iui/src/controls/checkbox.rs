use std::rc::Rc;

use iui::UI;
use regui::{decl_function_component, function_component::{Cx, ComponentFunction}};

use crate::Control;

pub struct CheckboxProps {
    pub ui: UI,
    pub text: String,
    pub checked: bool,
    pub on_toggled: Rc<dyn Fn(bool)>,
    pub enabled: bool,
}

impl CheckboxProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            text: "".into(),
            checked: false,
            on_toggled: Rc::new(|_btn| {}),
            enabled: true,
        }
    }
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.into();
        self
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn on_toggled(mut self, on_toggled: impl Fn(bool) + 'static) -> Self {
        self.on_toggled = Rc::new(on_toggled);
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        Checkbox::eval(cx, self)
    }
}

decl_function_component!(pub Checkbox checkbox(CheckboxProps) -> Control);

impl Checkbox {
    pub fn builder(ui: &UI) -> CheckboxProps {
        CheckboxProps::new(ui)
    }
}

fn checkbox(props: &CheckboxProps, cx: &mut Cx) -> Control {
    let checkbox = cx.use_state(|| iui::controls::Checkbox::new(&props.ui, &props.text));
    let control = cx.use_state(|| Control::new(checkbox.get()));

    let mut checkbox = checkbox.get();

    // TODO set text

    checkbox.set_checked(&props.ui, props.checked);

    checkbox.on_toggled(&props.ui, {
        let on_toggled = props.on_toggled.clone();
        move |checked| {
            on_toggled(checked);
        }
    });

    if props.enabled {
        checkbox.enable(&props.ui);
    } else {
        checkbox.disable(&props.ui);
    }

    control.get()
}