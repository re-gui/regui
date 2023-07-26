use iui::UI;
use regui::{decl_function_component, function_component::{Cx, ComponentFunction}};

use crate::Control;

pub struct SpacerProps {
    pub ui: UI,
}

impl SpacerProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
        }
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        Spacer::eval(cx, self)
    }
}

decl_function_component!(pub Spacer spacer(SpacerProps) -> Control);

impl Spacer {
    pub fn builder(ui: &UI) -> SpacerProps {
        SpacerProps::new(ui)
    }
}

fn spacer(props: &SpacerProps, cx: &mut Cx) -> Control {
    let spacer = cx.use_state(|| iui::controls::HorizontalSeparator::new(&props.ui));
    let control = cx.use_ref(|| Control::new(spacer.get()));

    (*control).clone()
}