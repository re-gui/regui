
use std::{rc::Rc, cell::RefCell};

use native_windows_gui as nwg;
use regui::{StateFunction, component::{FunctionsCache, EvalFromCache}};

use crate::{WithNwgControlHandle, NativeCommonComponentComponent, NwgControlNode, NativeCommonComponent};

impl WithNwgControlHandle for nwg::Button {
    fn nwg_control_handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
}

#[derive(Clone)]
pub struct ButtonProps {
    pub id: Option<i32>,
    pub text: String, // TODO cow
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    pub enabled: bool,
    pub on_click: Rc<dyn Fn()>,
    // TODO font etc.
}

impl Default for ButtonProps {
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

pub struct ButtonPropsBuilder {
    props: ButtonProps,
}

impl ButtonPropsBuilder {
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

    pub fn on_click(mut self, on_click: impl Fn() + 'static) -> Self {
        self.props.on_click = Rc::new(on_click);
        self
    }

    pub fn build_props(self) -> ButtonProps {
        self.props
    }
}

impl EvalFromCache for ButtonPropsBuilder {
    type Out = NwgControlNode;
    fn eval(self, cache: &FunctionsCache) -> Self::Out {
        cache.eval::<Button>(self.build_props())
    }
}

pub struct Button {
    native: NativeCommonComponentComponent<nwg::Button>,
    on_click_ref: Rc<RefCell<Rc<dyn Fn()>>>,
    props: ButtonProps,
}

impl Button {
    pub fn builder() -> ButtonPropsBuilder {
        ButtonPropsBuilder {
            props: ButtonProps::default(),
        }
    }
}


impl StateFunction for Button {
    type Input = ButtonProps;
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