use std::{rc::Rc, cell::RefCell};

use native_windows_gui as nwg;
use regui::{component::{GetFromCache, FunctionsCache}, StateFunction};

use crate::{WithNwgControlHandle, NwgControlNode, NativeCommonComponent, NativeCommonComponentComponent, ControlEvent};



impl WithNwgControlHandle for nwg::ExternCanvas {
    fn nwg_control_handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
}

#[derive(Clone)]
pub struct ExternCanvasProps {
    pub id: Option<i32>,
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    pub enabled: bool,
    pub on_event: Rc<dyn Fn(&ControlEvent)>,
    pub on_created: Rc<dyn Fn(&nwg::ControlHandle)>,
    // TODO ...
}

impl Default for ExternCanvasProps {
    fn default() -> Self {
        Self {
            id: None,
            position: None,
            size: None,
            enabled: true,
            on_event: Rc::new(|_| {}),
            on_created: Rc::new(|_| {}),
        }
    }
}

pub struct ExternCanvasPropsBuilder {
    props: ExternCanvasProps,
}

impl ExternCanvasPropsBuilder {
    pub fn id(mut self, id: i32) -> Self {
        self.props.id = Some(id);
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

    pub fn on_event(mut self, on_event: impl Fn(&ControlEvent) + 'static) -> Self {
        self.props.on_event = Rc::new(on_event);
        self
    }

    pub fn on_created(mut self, on_created: impl Fn(&nwg::ControlHandle) + 'static) -> Self {
        self.props.on_created = Rc::new(on_created);
        self
    }

    pub fn build_props(self) -> ExternCanvasProps {
        self.props
    }
}

impl GetFromCache for ExternCanvasPropsBuilder {
    type Out = NwgControlNode;
    fn get(self, cache: &FunctionsCache) -> Self::Out {
        cache.eval::<ExternCanvas>(self.build_props())
    }
}

pub struct ExternCanvas {
    native: NativeCommonComponentComponent<nwg::ExternCanvas>,
    on_event_ref: Rc<RefCell<Rc<dyn Fn(&ControlEvent)>>>,
    on_created_ref: Rc<RefCell<Rc<dyn Fn(&nwg::ControlHandle)>>>,
    props: ExternCanvasProps,
}

impl ExternCanvas {
    pub fn builder() -> ExternCanvasPropsBuilder {
        ExternCanvasPropsBuilder {
            props: ExternCanvasProps::default(),
        }
    }
}

impl StateFunction for ExternCanvas {
    type Input = ExternCanvasProps;
    type Output = NwgControlNode;
    fn build(input: Self::Input) -> (Self::Output, Self) {
        let on_event_ref = Rc::new(RefCell::new(input.on_event.clone()));
        let on_created_ref = Rc::new(RefCell::new(input.on_created.clone()));
        let (node, native) = NativeCommonComponentComponent::build(NativeCommonComponent {
            build: Rc::new({
                let input = input.clone();
                let on_created_ref = on_created_ref.clone();
                move |parent| {
                    let canvas = build_nwg_canvas(parent, &input);
                    let on_created = on_created_ref.borrow().clone();
                    on_created(&canvas.handle);
                    canvas
                }
            }),
            on_native_event: Rc::new(|_, _, _, _| {}),
            on_event: Rc::new({
                let on_event_ref = on_event_ref.clone();
                move |event| {
                    let on_event = on_event_ref.borrow().clone();
                    on_event(event);
                }
            }),
        });

        (
            node,
            Self {
                native,
                on_event_ref,
                on_created_ref,
                props: input,
            }
        )
    }
    fn changed(&mut self, props: Self::Input) -> Self::Output {
        self.native.if_control(|canvas| {
            if props.position != self.props.position {
                if let Some((x, y)) = props.position {
                    canvas.set_position(x, y);
                }
            }

            if props.size != self.props.size {
                if let Some((w, h)) = props.size {
                    canvas.set_size(w, h);
                }
            }

            if props.enabled != self.props.enabled {
                canvas.set_enabled(props.enabled);
            }

            if !Rc::ptr_eq(&props.on_event, &self.props.on_event) {
                *self.on_event_ref.borrow_mut() = props.on_event.clone();
            }

            if !Rc::ptr_eq(&props.on_created, &self.props.on_created) {
                *self.on_created_ref.borrow_mut() = props.on_created.clone();
            }
        });
        self.props = props;
        self.native.get_node()
    }
    // TODO reuse_with
}

fn build_nwg_canvas(parent: &nwg::ControlHandle, props: &ExternCanvasProps) -> nwg::ExternCanvas {
    let mut canvas = Default::default();
    let mut builder = nwg::ExternCanvas::builder()
        .parent(Some(parent));

        if let Some(position) = props.position {
            builder = builder.position(position);
        }

        if let Some(size) = props.size {
            builder = builder.size((size.0 as i32, size.1 as i32));
        }

    builder
        .build(&mut canvas)
        .expect("Failed to build label");

    canvas.set_enabled(props.enabled);

    canvas
}