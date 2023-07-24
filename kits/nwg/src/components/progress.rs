use std::rc::Rc;

use crate::{WithNwgControlHandle, NwgNode};
use native_windows_gui as nwg;
use regui::{component::{GetFromCache, FunctionsCache}, StateFunction};

use super::{NativeCommonComponent, NativeCommonComponentProps};

impl WithNwgControlHandle for nwg::ProgressBar {
    fn nwg_control_handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
    fn position(&self) -> (i32, i32) {
        self.position()
    }
    fn size(&self) -> (u32, u32) {
        self.size()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressBarProps {
    pub id: Option<i32>,
    pub value: f32,
    pub position: Option<(i32, i32)>,
    pub size: Option<(i32, i32)>,
}

impl Default for ProgressBarProps {
    fn default() -> Self {
        Self {
            id: None,
            value: 0.42,
            position: None,
            size: None,
        }
    }
}

pub struct ProgressBarPropsBuilder {
    props: ProgressBarProps,
}

impl ProgressBarPropsBuilder {
    pub fn id(mut self, id: i32) -> Self {
        self.props.id = Some(id);
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.props.value = value;
        self
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.props.position = Some((x, y));
        self
    }

    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.props.size = Some((width, height));
        self
    }

    pub fn build_props(self) -> ProgressBarProps {
        self.props
    }
}

impl GetFromCache for ProgressBarPropsBuilder {
    type Out = NwgNode<nwg::ControlHandle>;
    fn get(self, cache: &FunctionsCache) -> Self::Out {
        cache.eval::<ProgressBar>(self.build_props())
    }
}

pub struct ProgressBar {
    native: NativeCommonComponent<nwg::ProgressBar>,
    props: ProgressBarProps,
}

impl ProgressBar {
    pub fn builder() -> ProgressBarPropsBuilder {
        ProgressBarPropsBuilder {
            props: ProgressBarProps::default(),
        }
    }
}

impl StateFunction for ProgressBar {
    type Input = ProgressBarProps;
    type Output = NwgNode<nwg::ControlHandle>;
    fn build(input: Self::Input) -> (Self::Output, Self) {
        let (node, native) = NativeCommonComponent::build(NativeCommonComponentProps {
            build: Rc::new({
                let props = input.clone();
                move |parent| build_nwg_progress_bar(parent, &props)
            }),
            on_native_event: Rc::new(|_, _, _, _| {}),
            on_event: Rc::new(|_| {}), // TODO
        });

        (
            node,
            Self {
                native,
                props: input,
            }
        )
    }
    fn changed(&mut self, input: Self::Input) -> Self::Output {
        self.native.if_control(|label| update_nwg_progress_bar(label, &input, &self.props));
        self.props = input;
        self.native.get_node()
    }
    // TODO reuse_with
}

fn build_nwg_progress_bar(
    parent: &nwg::ControlHandle,
    props: &ProgressBarProps
) -> nwg::ProgressBar {
    let mut label = Default::default();
    let mut builder = nwg::ProgressBar::builder()
        .range(0..1000)
        .pos((props.value * 1000.0) as u32)
        .parent(parent);

    if let Some(position) = props.position {
        builder = builder.position(position);
    }

    if let Some(size) = props.size {
        builder = builder.size((size.0 as i32, size.1 as i32));
    }

    builder
        .build(&mut label)
        .expect("Failed to build label");

    label
}

fn update_nwg_progress_bar(
    label: &nwg::ProgressBar,
    props: &ProgressBarProps,
    old_props: &ProgressBarProps
) {
    if props.value != old_props.value {
        label.set_pos((props.value * 1000.0) as u32);
    }

    if props.position != old_props.position {
        if let Some((x, y)) = props.position {
            label.set_position(x, y);
        }
    }

    if props.size != old_props.size {
        if let Some((w, h)) = props.size {
            label.set_size(w as u32, h as u32);
        }
    }
}