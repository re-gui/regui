
use std::{rc::Rc, cell::RefCell};

use native_windows_gui as nwg;
use regui::{StateFunction, component::{FunctionsCache, GetFromCache}};

use crate::{WithNwgControlHandle, NwgNode};

use super::{NativeCommonComponent, NativeCommonComponentProps};

impl WithNwgControlHandle for nwg::TextInput {
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

#[derive(Clone)]
pub struct TextInputProps {
    pub id: Option<i32>,
    pub text: String, // TODO cow
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    pub enabled: bool,
    pub on_input: Rc<dyn Fn(&str)>,
    pub on_user_input: Rc<dyn Fn(&str)>,
    // TODO on_user_input that ignores programmatic input
    // TODO font etc.
}

impl Default for TextInputProps {
    fn default() -> Self {
        Self {
            id: None,
            text: "".into(),
            position: None,
            size: None,
            enabled: true,
            on_input: Rc::new(|_| {}),
            on_user_input: Rc::new(|_| {}),
        }
    }
}

pub struct TextInputPropsBuilder {
    props: TextInputProps,
}

impl TextInputPropsBuilder {
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

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.props.size = Some((width, height));
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.props.enabled = enabled;
        self
    }

    pub fn on_input(mut self, on_input: impl Fn(&str) + 'static) -> Self {
        self.props.on_input = Rc::new(on_input);
        self
    }

    pub fn on_user_input(mut self, on_user_input: impl Fn(&str) + 'static) -> Self {
        self.props.on_user_input = Rc::new(on_user_input);
        self
    }

    pub fn build_props(self) -> TextInputProps {
        self.props
    }
}

impl GetFromCache for TextInputPropsBuilder {
    type Out = NwgNode<nwg::ControlHandle>;
    fn get(self, cache: &FunctionsCache) -> Self::Out {
        cache.eval::<TextInput>(self.build_props())
    }
}

pub struct TextInput {
    native: NativeCommonComponent<nwg::TextInput>,
    on_input_ref: Rc<RefCell<Rc<dyn Fn(&str)>>>,
    on_user_input_ref: Rc<RefCell<Rc<dyn Fn(&str)>>>,
    programmatic_setting: Rc<RefCell<bool>>,
    props: TextInputProps,
}

impl TextInput {
    pub fn builder() -> TextInputPropsBuilder {
        TextInputPropsBuilder {
            props: TextInputProps::default(),
        }
    }
}

impl StateFunction for TextInput {
    type Input = TextInputProps;
    type Output = NwgNode<nwg::ControlHandle>;

    fn build(props: Self::Input) -> (Self::Output, Self) {
        let on_input_ref = Rc::new(RefCell::new(props.on_input.clone()));
        let on_user_input_ref = Rc::new(RefCell::new(props.on_user_input.clone()));
        let programmatic_setting = Rc::new(RefCell::new(false));
        let (node, native) = NativeCommonComponent::build(NativeCommonComponentProps {
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

                    label.set_enabled(props.enabled);

                    label
                }
            }),
            on_native_event: Rc::new({
                let on_input_ref = on_input_ref.clone();
                let on_user_input_ref = on_user_input_ref.clone();
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
                                let on_user_input = on_user_input_ref.borrow().clone();
                                on_user_input(&new_text);
                            }
                            let on_input = on_input_ref.borrow().clone();
                            on_input(&new_text);
                        }
                    }
                }
            }),
            on_event: Rc::new(|_| {}), // TODO
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

            if props.enabled != self.props.enabled {
                label.set_enabled(props.enabled);
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
    // TODO reuse_with
}