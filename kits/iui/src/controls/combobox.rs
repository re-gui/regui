use std::rc::Rc;

use iui::UI;
use regui::{decl_function_component, function_component::{Cx, ComponentFunction}};

use crate::Control;

pub struct ComboboxProps {
    pub ui: UI,
    pub items: Vec<String>,
    pub selected: usize,
    pub on_selected: Rc<dyn Fn(usize)>,
    pub enabled: bool,
}

impl ComboboxProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            items: vec![],
            selected: 0,
            on_selected: Rc::new(|_btn| {}),
            enabled: true,
        }
    }

    pub fn items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }
    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }
    pub fn on_selected(mut self, on_selected: impl Fn(usize) + 'static) -> Self {
        self.on_selected = Rc::new(on_selected);
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        Combobox::eval(cx, self)
    }
}

decl_function_component!(pub Combobox combobox(ComboboxProps) -> Control);

impl Combobox {
    pub fn builder(ui: &UI) -> ComboboxProps {
        ComboboxProps::new(ui)
    }
}

fn combobox(props: &ComboboxProps, cx: &mut Cx) -> Control {
    let combobox = cx.use_state(|| {
        let combo = iui::controls::Combobox::new(&props.ui);

        for item in &props.items {
            combo.append(&props.ui, &item);
        }

        combo
    });
    let control = cx.use_state(|| Control::new(combobox.get()));

    let mut combobox = combobox.get();

    let old_items = cx.use_state(|| props.items.clone());
    if old_items.get() != props.items {
        todo!("Combobox items change not implemented");
    }
    old_items.set(props.items.clone());

    combobox.set_selected(&props.ui, props.selected as i32);

    combobox.on_selected(&props.ui, {
        let on_selected = props.on_selected.clone();
        move |selected| {
            on_selected(selected as usize);
        }
    });

    control.get()
}