use std::rc::Rc;

use regui::{decl_function_component, function_component::{Cx, ComponentFunction}};
use iui::{controls::Button as IuiButton, UI};

use crate::Control;

pub struct ButtonProps {
    pub ui: UI,
    pub text: String,
    pub on_click: Rc<dyn Fn(&IuiButton)>,
    pub enabled: bool,
}

impl ButtonProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            text: "".into(),
            on_click: Rc::new(|_btn| {}),
            enabled: true,
        }
    }
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.into();
        self
    }
    pub fn on_click(mut self, on_click: impl Fn(&IuiButton) + 'static) -> Self {
        self.on_click = Rc::new(on_click);
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        Button::eval(cx, self)
    }
}

decl_function_component!(pub Button button(ButtonProps) -> Control);

impl Button {
    pub fn builder(ui: &UI) -> ButtonProps {
        ButtonProps::new(ui)
    }
}

fn button(props: &ButtonProps, cx: &mut Cx) -> Control {
    let button = cx.use_state(|| IuiButton::new(&props.ui, &props.text));
    let control = cx.use_ref(|| Control::new(button.get()));

    let mut button = button.get();

    button.set_text(&props.ui, &props.text);

    button.on_clicked(&props.ui, {
        let on_click = props.on_click.clone();
        move |btn| {
            on_click(btn);
        }
    });

    if props.enabled {
        button.enable(&props.ui);
    } else {
        button.disable(&props.ui);
    }

    (*control).clone()
}