
use std::{rc::Rc, cell::RefCell};

use native_windows_gui as nwg;
use regui::{StateFunctionProps, StateFunction};

use crate::{WithNwgControlHandle, NativeCommonComponentComponent, NwgControlNode, NativeCommonComponent};

impl WithNwgControlHandle for nwg::Button {
    fn nwg_control_handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
}

#[derive(Clone)]
pub struct Button {
    pub id: Option<i32>,
    pub text: String, // TODO cow
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    pub enabled: bool,
    pub on_click: Rc<dyn Fn()>,
    // TODO font etc.
}

impl Default for Button {
    fn default() -> Self {
        Self {
            id: None,
            text: "".into(),
            position: None,
            size: None,
            enabled: true,
            on_click: Rc::new(|| {}),
        }
    }
}

impl StateFunctionProps for Button {
    type AssociatedFunction = ButtonFunction;
}

pub struct ButtonFunction {
    native: NativeCommonComponentComponent<nwg::Button>,
    on_click_ref: Rc<RefCell<Rc<dyn Fn()>>>,
    props: Button,
}

impl StateFunction for ButtonFunction {
    type Input = Button;
    type Output = NwgControlNode;

    fn build(props: Self::Input) -> (Self::Output, Self) {
        let on_click_ref = Rc::new(RefCell::new(props.on_click.clone()));
        let (node, native) = NativeCommonComponentComponent::build(NativeCommonComponent {
            build: Rc::new({
                let props = props.clone();
                move |parent| {
                    let mut label = Default::default();
                    let mut builder = nwg::Button::builder()
                        .text(&props.text)
                        .parent(parent);

                        if let Some(position) = props.position {
                            builder = builder.position(position);
                        }
    
                        if let Some(size) = props.size {
                            builder = builder.size((size.0 as i32, size.1 as i32));
                        }

                        builder = builder.enabled(props.enabled);

                    builder
                        .build(&mut label)
                        .expect("Failed to build label");
                    label
                }
            }),
            on_event: Rc::new({
                let on_click_ref = on_click_ref.borrow().clone();
                move |event, _evt_data, _handle, _control| {
                    if let nwg::Event::OnButtonClick = event {
                        on_click_ref();
                    }
                }
            }),
        });

        (
            node,
            Self {
                native,
                on_click_ref,
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

            if props.enabled != self.props.enabled {
                label.set_enabled(props.enabled);
            }

            if !Rc::ptr_eq(&props.on_click, &self.props.on_click) {
                *self.on_click_ref.borrow_mut() = props.on_click.clone();
            }
        });
        self.props = props;
        self.native.get_node()
    }
}