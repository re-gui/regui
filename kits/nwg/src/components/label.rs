
use std::rc::Rc;

use native_windows_gui as nwg;
use regui::{StateFunctionProps, StateFunction};

use crate::{WithNwgControlHandle, NativeCommonComponentComponent, NwgControlNode, NativeCommonComponent};

impl WithNwgControlHandle for nwg::Label {
    fn nwg_control_handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub id: Option<i32>,
    pub text: String, // TODO cow
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    // TODO font etc.
}

impl Default for Label {
    fn default() -> Self {
        Self {
            id: None,
            text: "".into(),
            position: None,
            size: None,
        }
    }
}

impl StateFunctionProps for Label {
    type AssociatedFunction = LabelFunction;
}

pub struct LabelFunction {
    native: NativeCommonComponentComponent<nwg::Label>,
    props: Label,
}

impl StateFunction for LabelFunction {
    type Input = Label;
    type Output = NwgControlNode;

    fn build(props: Self::Input) -> (Self::Output, Self) {
        let (node, native) = NativeCommonComponentComponent::build(NativeCommonComponent {
            build: Rc::new({
                let props = props.clone();
                move |parent| {
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
            }),
            on_event: Rc::new(|_event, _evt_data, _handle, _control| {}),
        });

        (
            node,
            Self {
                native,
                props,
            }
        )
    }
    fn changed(&mut self, props: Self::Input) -> Self::Output {
        self.native.if_control(|label| {
            if props.text != self.props.text {
                label.set_text(&props.text);
            }

            if props.position != self.props.position {
                if let Some((x, y)) = props.position {
                    label.set_position(x, y);
                }
            }

            if props.size != self.props.size {
                if let Some((w, h)) = props.size {
                    label.set_size(w, h);
                }
            }
        });
        self.props = props;
        self.native.get_node()
    }
}