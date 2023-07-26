use std::{rc::Rc, cell::RefCell};

use iui::{UI, prelude::TextEntry};
use regui::{decl_function_component, function_component::{Cx, ComponentFunction}};

use crate::Control;

pub struct MultilineEntryProps {
    pub ui: UI,
    pub text: String,
    pub on_changed: Rc<dyn Fn(String)>,
    pub enabled: bool,
}

impl MultilineEntryProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            text: "".into(),
            on_changed: Rc::new(|_btn| {}),
            enabled: true,
        }
    }
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.into();
        self
    }
    pub fn on_changed(mut self, on_changed: impl Fn(String) + 'static) -> Self {
        self.on_changed = Rc::new(on_changed);
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        MultilineEntry::eval(cx, self)
    }
}

decl_function_component!(pub MultilineEntry multiline_entry(MultilineEntryProps) -> Control);

impl MultilineEntry {
    pub fn builder(ui: &UI) -> MultilineEntryProps {
        MultilineEntryProps::new(ui)
    }
}

fn multiline_entry(props: &MultilineEntryProps, cx: &mut Cx) -> Control {
    let multiline_entry = cx.use_state(|| iui::controls::MultilineEntry::new(&props.ui));
    let control = cx.use_state(|| Control::new(multiline_entry.get()));

    let mut multiline_entry = multiline_entry.get();

    let old_text = cx.use_ref(|| RefCell::new(props.text.clone()));
    if *old_text.borrow() == props.text  {
        multiline_entry.set_value(&props.ui, &props.text);
        *old_text.borrow_mut() = props.text.clone();
    }

    multiline_entry.on_changed(&props.ui, {
        let on_changed = props.on_changed.clone();
        move |entry| {
            on_changed(entry);
        }
    });

    if props.enabled {
        multiline_entry.enable(&props.ui);
    } else {
        multiline_entry.disable(&props.ui);
    }

    control.get()
}