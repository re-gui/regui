use iui::UI;
use regui::{decl_function_component, function_component::{Cx, ComponentFunction}};

use crate::Control;

#[derive(Clone, PartialEq)]
pub struct LayoutPosition {
    pub left: i32,
    pub height: i32,
    pub x_span: i32,
    pub y_span: i32,
    pub x_expand: bool,
    pub y_expand: bool,
    pub h_align: iui::controls::GridAlignment,
    pub v_align: iui::controls::GridAlignment,
}

impl LayoutPosition {
    pub fn unit(x: i32, y: i32) -> Self {
        Self {
            left: x,
            height: y,
            x_span: 1,
            y_span: 1,
            x_expand: false,
            y_expand: false,
            h_align: iui::controls::GridAlignment::Fill,
            v_align: iui::controls::GridAlignment::Fill,
        }
    }
    pub fn xywh(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            left: x,
            height: y,
            x_span: w,
            y_span: h,
            x_expand: false,
            y_expand: false,
            h_align: iui::controls::GridAlignment::Fill,
            v_align: iui::controls::GridAlignment::Fill,
        }
    }
}

pub struct LayoutGridProps {
    pub ui: UI,
    pub enable: bool,
    pub padded: bool,
    pub children: Vec<(Control, LayoutPosition)>,
}

impl LayoutGridProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            enable: true,
            padded: false,
            children: vec![],
        }
    }
    pub fn enable(mut self, enable: bool) -> Self {
        self.enable = enable;
        self
    }
    pub fn padded(mut self, padded: bool) -> Self {
        self.padded = padded;
        self
    }
    pub fn child(mut self, child: Control, position: LayoutPosition) -> Self {
        self.children.push((child, position));
        self
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        LayoutGrid::eval(cx, self)
    }
}

decl_function_component!(pub LayoutGrid layout_grid(LayoutGridProps) -> Control);

impl LayoutGrid {
    pub fn builder(ui: &UI) -> LayoutGridProps {
        LayoutGridProps::new(ui)
    }
}

fn layout_grid(props: &LayoutGridProps, cx: &mut Cx) -> Control {
    let layout_grid = cx.use_state(|| {
        let mut layout_grid = iui::controls::LayoutGrid::new(&props.ui);
        for (child, position) in &props.children {
            layout_grid.append(
                &props.ui,
                (*child.control).clone(),
                position.left,
                position.height,
                position.x_span,
                position.y_span,
                match (position.x_expand, position.y_expand) {
                    (true, true) => iui::controls::GridExpand::Both,
                    (true, false) => iui::controls::GridExpand::Horizontal,
                    (false, true) => iui::controls::GridExpand::Vertical,
                    (false, false) => iui::controls::GridExpand::Neither,
                },
                position.h_align,
                position.v_align,
            );
        }
        layout_grid
    });
    let control = cx.use_ref(|| Control::new(layout_grid.get()));

    let mut layout_grid = layout_grid.get();

    let old_children = cx.use_state(|| props.children.clone());

    if props.children != old_children.get() {
        todo!()
    }

    old_children.set(props.children.clone());

    layout_grid.set_padded(&props.ui, props.padded);

    (*control).clone()
}