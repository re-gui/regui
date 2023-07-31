use std::ops::Deref;

use iui::{UI, prelude::LayoutStrategy};
use iui::controls::HorizontalBox as IuiHorizontalBox;
use regui::function_component::{Cx, ComponentFunction};
use regui::decl_function_component;

use crate::Control;

pub struct Strategy(LayoutStrategy);

impl Clone for Strategy {
    fn clone(&self) -> Self {
        Self(match self.0 {
            LayoutStrategy::Compact => LayoutStrategy::Compact,
            LayoutStrategy::Stretchy => LayoutStrategy::Stretchy,
        })
    }
}

impl PartialEq for Strategy {
    fn eq(&self, other: &Self) -> bool {
        match self.0 {
            LayoutStrategy::Compact => match other.0 {
                LayoutStrategy::Compact => true,
                _ => false,
            },
            LayoutStrategy::Stretchy => match other.0 {
                LayoutStrategy::Stretchy => true,
                _ => false,
            },
        }
    }
}

pub struct VertialBoxProps {
    pub ui: UI,
    pub padded: bool,
    pub children: Vec<(Control, Strategy)>,
}

impl VertialBoxProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            padded: false,
            children: vec![],
        }
    }
    pub fn padded(mut self, padded: bool) -> Self {
        self.padded = padded;
        self
    }
    pub fn child(mut self, child: Control, layout_strategy: LayoutStrategy) -> Self {
        self.children.push((child, Strategy(layout_strategy)));
        self
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        HorizontalBox::eval(cx, self)
    }
}

decl_function_component!(pub HorizontalBox horizontal_box(VertialBoxProps) -> Control);

impl HorizontalBox {
    pub fn builder(ui: &UI) -> VertialBoxProps {
        VertialBoxProps::new(ui)
    }
}

fn horizontal_box(props: &VertialBoxProps, cx: &mut Cx) -> Control {
    let hbox = cx.use_state(|| {
        let mut hbox = IuiHorizontalBox::new(&props.ui);
        for (child, layout_strategy) in &props.children {
            hbox.append(&props.ui, child.control.deref().clone(), layout_strategy.clone().0);
        }
        hbox
    });
    let control = cx.use_ref(|| Control::new(hbox.get()));

    let mut hbox = hbox.get();

    hbox.set_padded(&props.ui, props.padded);

    let old_children = cx.use_state(|| props.children.clone());

    if props.children != old_children.get() {
        todo!()
    }

    old_children.set(props.children.clone());

    control.deref().clone()
}