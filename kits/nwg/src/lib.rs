
use std::{rc::Rc, cell::{RefCell, Cell}, ops::Deref};

use native_windows_gui as nwg;

use regui::{StateFunction, component::{Component, LiveStateComponent}};

//use crate::Callback;

pub mod components;

pub fn run_ui<UiComponent: Component>(props: UiComponent::Props) {
    let (_out, _component) = LiveStateComponent::<UiComponent>::build(props);
    nwg::dispatch_thread_events();
}

/// A [`native_windows_gui`] [Common Control](https://learn.microsoft.com/en-us/windows/win32/controls/common-controls-intro)
pub trait WithNwgControlHandle: 'static {
    fn nwg_control_handle(&self) -> &nwg::ControlHandle;
}

pub struct NCCData<Control: WithNwgControlHandle> {
    parent_handle: nwg::ControlHandle,
    handler: nwg::EventHandler,
    component: Rc<Control>,
}

impl<Control: WithNwgControlHandle> Drop for NCCData<Control> {
    fn drop(&mut self) {
        // TODO remove from parent??

        // NOTE: the handler has to be the first member, because it is dropped first.
        // handler cannot be dropped after the control is dropped. `unbind_event_handler` panics if the control is dropped.
        nwg::unbind_event_handler(&self.handler);
    }
}

pub struct NativeCommonComponent<Control: WithNwgControlHandle> {
    pub build: Rc<dyn Fn(&nwg::ControlHandle) -> Control>,
    pub on_native_event: Rc<dyn Fn(&nwg::Event, &nwg::EventData, &nwg::ControlHandle, &Control)>,
    pub on_event: Rc<dyn Fn(&ControlEvent)>,
}

pub struct NativeCommonComponentComponent<Control: WithNwgControlHandle> {
    //data: Rc<RefCell<Option<NCCData<Control>>>>,
    data: Rc<RefCell<Option<Rc<NCCData<Control>>>>>,
    node: Option<NwgControlNode>,
    props: NativeCommonComponent<Control>,
}

impl<Control: WithNwgControlHandle> NativeCommonComponentComponent<Control> {
    pub fn if_control<F: FnOnce(&Control)>(&self, f: F) {
        if let Some(data) = self.data.borrow().as_ref() {
            f(&data.component);
        }
    }
    pub fn get_node(&mut self) -> NwgControlNode {
        if let Some(node) = self.node.clone() {
            node
        } else {
            let node = NwgControlNode(Rc::new(RefCell::new(NativeCommonComponentNode {
                data: self.data.clone(),
                on_native_event: self.props.on_native_event.clone(),
                on_event: self.props.on_event.clone(),
                build: self.props.build.clone(),
            })));
            self.node = Some(node.clone());
            node
        }
    }
}

impl<Control: WithNwgControlHandle> StateFunction for NativeCommonComponentComponent<Control> {
    type Input = NativeCommonComponent<Control>;
    type Output = NwgControlNode;
    fn build(props: Self::Input) -> (Self::Output, Self) {
        let mut component = Self {
            data: Rc::new(RefCell::new(None)),
            node: None,
            props: props,
        };

        let node = component.get_node();

        (node, component)
    }
    fn changed(&mut self, props: Self::Input) -> Self::Output {
        // TODO check if props changed, especially on_event
        self.props = props;
        self.get_node()
    }
}

pub struct NativeCommonComponentNode<Control: WithNwgControlHandle> {
    data: Rc<RefCell<Option<Rc<NCCData<Control>>>>>,
    on_native_event: Rc<dyn Fn(&nwg::Event, &nwg::EventData, &nwg::ControlHandle, &Control)>, // TODO double Rc
    on_event: Rc<dyn Fn(&ControlEvent)>,
    build: Rc<dyn Fn(&nwg::ControlHandle) -> Control>,
}

impl<Control: WithNwgControlHandle> NwgControlNodeTrait for NativeCommonComponentNode<Control> {
    fn handle_from_parent(&mut self, parent_handle: &nwg::ControlHandle) -> nwg::ControlHandle {
        let data = {
            let data = self.data.borrow();
            if let Some(current_data) = data.clone() {
                if current_data.parent_handle == *parent_handle {
                    Some(current_data)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let data = if let Some(data) = data {
            data.clone()
        } else {
            let control = Rc::new((self.build)(parent_handle));

            let handler = {
                let on_native_event = self.on_native_event.clone();
                let on_event = self.on_event.clone();
                let control_handle = control.nwg_control_handle().clone();
                let control_weak = Rc::downgrade(&control);
                nwg::bind_event_handler(
                    &control.nwg_control_handle(),
                    parent_handle,
                    move |event, event_data, handle| {
                        if handle == control_handle {
                            if let Some(control) = control_weak.upgrade() {
                                (on_native_event)(&event, &event_data, &handle, &control);
                            }
                            let event = ControlEvent::from_nwg_event(&event, &event_data, &handle);
                            if let Some(event) = event {
                                (on_event)(&event);
                            }
                        }
                    }
                )
            };

            let data = NCCData {
                parent_handle: parent_handle.clone(),
                component: control,
                handler,
            };

            let data = Rc::new(data);
            *self.data.borrow_mut() = Some(data.clone());
            data
        };

        data.component.nwg_control_handle().clone()
    }
}

impl<Control: WithNwgControlHandle> NwgControlRefData for NCCData<Control> {
    fn native(&self) -> &dyn WithNwgControlHandle {
        self.component.as_ref()
    }
}

trait NwgControlRefData {
    #[must_use]
    fn native(&self) -> &dyn WithNwgControlHandle;
}

pub trait NwgControlNodeTrait {
    #[must_use]
    fn handle_from_parent(&mut self, parent_handle: &nwg::ControlHandle) -> nwg::ControlHandle;
}

#[derive(Clone)]
pub struct NwgControlNode(pub Rc<RefCell<dyn NwgControlNodeTrait>>);

impl PartialEq for NwgControlNode {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Deref for NwgControlNode {
    type Target = Rc<RefCell<dyn NwgControlNodeTrait>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlEvent {
    Window(WindowEvent),
    Mouse(MouseEvent),
}

impl ControlEvent {
    pub fn from_nwg_event(event: &nwg::Event, event_data: &nwg::EventData, handle: &nwg::ControlHandle) -> Option<Self> {
        let mouse_global_pos = nwg::GlobalCursor::position();
        let mouse_local_pos = nwg::GlobalCursor::local_position(handle, Some(mouse_global_pos));
        match event {
            nwg::Event::OnWindowClose => Some(ControlEvent::Window(WindowEvent::CloseRequest)),
            nwg::Event::OnWindowMaximize => Some(ControlEvent::Window(WindowEvent::Maximize)),
            nwg::Event::OnWindowMinimize => Some(ControlEvent::Window(WindowEvent::Minimize)),
            nwg::Event::OnMove => None, // TODO
            nwg::Event::OnResize => None, // TODO
            nwg::Event::OnMouseMove => {
                let event = MouseEvent::Move {
                    x: mouse_local_pos.0,
                    y: mouse_local_pos.1,
                    global_x: mouse_global_pos.0,
                    global_y: mouse_global_pos.1,
                };
                Some(ControlEvent::Mouse(event))
            }
            nwg::Event::OnMousePress(button) => {
                let event = match button {
                    nwg::MousePressEvent::MousePressLeftDown => MouseEvent::Pressed {
                        x: mouse_local_pos.0,
                        y: mouse_local_pos.1,
                        global_x: mouse_global_pos.0,
                        global_y: mouse_global_pos.1,
                    },
                    nwg::MousePressEvent::MousePressLeftUp => MouseEvent::Released {
                        x: mouse_local_pos.0,
                        y: mouse_local_pos.1,
                        global_x: mouse_global_pos.0,
                        global_y: mouse_global_pos.1,
                    },
                    nwg::MousePressEvent::MousePressRightDown => MouseEvent::Pressed {
                        x: mouse_local_pos.0,
                        y: mouse_local_pos.1,
                        global_x: mouse_global_pos.0,
                        global_y: mouse_global_pos.1,
                    },
                    nwg::MousePressEvent::MousePressRightUp => MouseEvent::Released {
                        x: mouse_local_pos.0,
                        y: mouse_local_pos.1,
                        global_x: mouse_global_pos.0,
                        global_y: mouse_global_pos.1,
                    },
                    // TODO middle? x1? x2?
                };
                Some(ControlEvent::Mouse(event))
            }
            nwg::Event::OnMouseWheel => {
                let delta = match event_data {
                    nwg::EventData::OnMouseWheel(delta) => *delta,
                    _ => panic!("Unexpected event data type")
                };
                let event = MouseEvent::Wheel {
                    delta,
                    x: mouse_local_pos.0,
                    y: mouse_local_pos.1,
                    global_x: mouse_global_pos.0,
                    global_y: mouse_global_pos.1,
                };
                Some(ControlEvent::Mouse(event))
            }
            _ => None,
        }
    }
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    CloseRequest,
    Maximize,
    Minimize,
    Moved(i32, i32),
    Resized(u32, u32),

    Paint,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MouseEvent {
    Pressed { x: i32, y: i32, global_x: i32, global_y: i32 },
    Released { x: i32, y: i32, global_x: i32, global_y: i32 },
    Move { x: i32, y: i32, global_x: i32, global_y: i32 }, // TODO delta
    Wheel { delta: i32, x: i32, y: i32, global_x: i32, global_y: i32 },
}