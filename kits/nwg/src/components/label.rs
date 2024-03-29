
use std::rc::Rc;

use native_windows_gui as nwg;
use regui::{StateFunction, component::{FunctionsCache, GetFromCache}};

use crate::{WithNwgControlHandle, NwgNode};

use super::{NativeCommonComponent, NativeCommonComponentProps};

impl WithNwgControlHandle for nwg::Label {
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
pub struct LabelProps {
    pub id: Option<i32>,
    pub text: String, // TODO cow
    pub position: Option<(i32, i32)>,
    pub size: Option<(i32, i32)>,
    // TODO font
    // TODO alignment
}

impl Default for LabelProps {
    fn default() -> Self {
        Self {
            id: None,
            text: "".into(),
            position: None,
            size: None,
        }
    }
}

pub struct LabelPropsBuilder {
    props: LabelProps,
}

impl LabelPropsBuilder {
    pub fn id(mut self, id: i32) -> Self {
        self.props.id = Some(id);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.props.text = text.into();
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

    pub fn build_props(self) -> LabelProps {
        self.props
    }
}

impl GetFromCache for LabelPropsBuilder {
    type Out = NwgNode<nwg::ControlHandle>;
    fn get(self, cache: &FunctionsCache) -> Self::Out {
        cache.eval::<Label>(self.build_props())
    }
}

pub struct Label {
    native: NativeCommonComponent<nwg::Label>,
    props: LabelProps,
}

impl Label {
    pub fn builder() -> LabelPropsBuilder {
        LabelPropsBuilder {
            props: LabelProps::default(),
        }
    }
}

impl StateFunction for Label {
    type Input = LabelProps;
    type Output = NwgNode<nwg::ControlHandle>;

    fn build(input: Self::Input) -> (Self::Output, Self) {
        let (node, native) = NativeCommonComponent::build(NativeCommonComponentProps {
            build: Rc::new({
                let props = input.clone();
                move |parent| build_nwg_label(parent, &props)
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
        self.native.if_control(|label| update_nwg_label(label, &input, &self.props));
        self.props = input;
        self.native.get_node()
    }
    // TODO reuse_with
}

fn build_nwg_label(
    parent: &nwg::ControlHandle,
    props: &LabelProps
) -> nwg::Label {
    let mut label = Default::default();
    let mut builder = nwg::Label::builder()
        .text(&props.text)
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

fn update_nwg_label(
    label: &nwg::Label,
    props: &LabelProps,
    old_props: &LabelProps
) {
    if props.text != old_props.text {
        label.set_text(&props.text);
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