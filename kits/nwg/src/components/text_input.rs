
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
    pub on_user_input: Rc<dyn Fn(&str)>,
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
            on_user_input: Rc::new(|_| {}),
        }
    }
}

impl StateFunctionProps for TextInput {
    type AssociatedFunction = TextInputFunction;
}

pub struct TextInputFunction {
    native: NativeCommonComponentComponent<nwg::TextInput>,
    on_input_ref: Rc<RefCell<Rc<dyn Fn(&str)>>>,
    on_user_input_ref: Rc<RefCell<Rc<dyn Fn(&str)>>>,
    programmatic_setting: Rc<RefCell<bool>>,
    props: TextInput,
}

impl StateFunction for TextInputFunction {
    type Input = TextInput;
    type Output = NwgControlNode;

    fn build(props: Self::Input) -> (Self::Output, Self) {
        let on_input_ref = Rc::new(RefCell::new(props.on_input.clone()));
        let on_user_input_ref = Rc::new(RefCell::new(props.on_user_input.clone()));
        let programmatic_setting = Rc::new(RefCell::new(false));
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
                let on_user_input_ref = on_user_input_ref.borrow().clone();
                let programmatic_setting = programmatic_setting.clone();
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
                            if !*programmatic_setting.borrow() {
                                on_user_input_ref(&new_text);
                            }
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
                on_input_ref,
                on_user_input_ref,
                programmatic_setting,
                props,
            }
        )
    }
    fn changed(&mut self, props: Self::Input) -> Self::Output {
        self.native.if_control(|label| {
            //if props.text != self.props.text {
                let current = label.text();
                if props.text != current {
                    *self.programmatic_setting.borrow_mut() = true;
                    label.set_text(&props.text);
                    *self.programmatic_setting.borrow_mut() = false;
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
                *self.on_input_ref.borrow_mut() = props.on_input.clone();
            }

            if !Rc::ptr_eq(&props.on_user_input, &self.props.on_user_input) {
                *self.on_user_input_ref.borrow_mut() = props.on_user_input.clone();
            }
        });
        self.props = props;
        self.native.get_node()
    }
}