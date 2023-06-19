
use std::{rc::Rc, cell::RefCell};

use native_windows_gui as nwg;
use regui::{StateFunctionProps, StateFunction};

use crate::{NwgNativeCommonControl, NativeCommonComponentComponent, NwgControlNode, NativeCommonComponent};

impl NwgNativeCommonControl for nwg::TextInput {
    fn handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
}

#[derive(Clone)]
pub struct TextInput {
    pub id: Option<i32>,
    pub text: String, // TODO cow
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    pub on_input: Rc<dyn Fn(&str)>,
    // TODO on_user_input that ignores programmatic input
    // TODO font etc.
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            id: None,
            text: "".into(),
            position: None,
            size: None,
            on_input: Rc::new(|_| {}),
        }
    }
}

impl StateFunctionProps for TextInput {
    type AssociatedComponent = TextInputComponent;
}

pub struct TextInputComponent {
    native: NativeCommonComponentComponent<nwg::TextInput>,
    on_click_ref: Rc<RefCell<Rc<dyn Fn(&str)>>>,
    props: TextInput,
}

impl StateFunction for TextInputComponent {
    type Input = TextInput;
    type Output = NwgControlNode;

    fn build(props: Self::Input) -> (Self::Output, Self) {
        let on_input_ref = Rc::new(RefCell::new(props.on_input.clone()));
        let (node, native) = NativeCommonComponentComponent::build(NativeCommonComponent {
            build: Rc::new({
                let props = props.clone();
                move |parent| {
                    let mut label = Default::default();
                    let mut builder = nwg::TextInput::builder()
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
            on_event: Rc::new({
                let on_input_ref = on_input_ref.borrow().clone();
                let current_text: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
                move |event, _evt_data, _handlem, control| {
                    if let nwg::Event::OnTextInput = event {
                        let new_text = control.text();
                        let changed = if let Some(current_text) = &*current_text.borrow() {
                            current_text != &new_text
                        } else {
                            true
                        };
                        if changed {
                            *current_text.borrow_mut() = Some(new_text.clone());
                            on_input_ref(&new_text);
                        }
                    }
                }
            }),
        });

        (
            node,
            Self {
                native,
                on_click_ref: on_input_ref,
                props,
            }
        )
    }
    fn changed(&mut self, props: Self::Input) -> Self::Output {
        self.native.if_control(|label| {
            //if props.text != self.props.text {
                let current = label.text();
                if props.text != current {
                    label.set_text(&props.text);
                }
            //}

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

            if !Rc::ptr_eq(&props.on_input, &self.props.on_input) {
                *self.on_click_ref.borrow_mut() = props.on_input.clone();
            }
        });
        self.props = props;
        self.native.get_node()
    }
}