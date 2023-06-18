
use std::{rc::{Rc, Weak}, cell::RefCell, ops::Deref};

use native_windows_gui as nwg;

use crate::state_function::StateFunction;

//use crate::Callback;

pub mod components;

/// A [`native_windows_gui`] [Common Control](https://learn.microsoft.com/en-us/windows/win32/controls/common-controls-intro)
pub trait NwgNativeCommonControl: 'static {
    fn handle(&self) -> &nwg::ControlHandle;
}

pub struct NwgHandler(nwg::EventHandler);

impl Drop for NwgHandler {
    fn drop(&mut self) {
        nwg::unbind_event_handler(&self.0);
    }
}

pub struct NCCData<Control: NwgNativeCommonControl> {
    handler: NwgHandler,
    parent_handle: nwg::ControlHandle,
    component: Control,
}

pub struct NativeCommonComponent<Control: NwgNativeCommonControl> {
    //parent_window: nwg::ControlHandle,
    //build: Callback<dyn Fn(nwg::ControlHandle) -> Control>,
    pub build: Rc<dyn Fn(&nwg::ControlHandle) -> Control>,
    //on_event: Option<Callback<dyn Fn(&nwg::Event, nwg::EventData, nwg::ControlHandle)>>,
    pub on_event: Rc<dyn Fn(&nwg::Event, &nwg::EventData, &nwg::ControlHandle)>,
}

pub struct NativeCommonComponentComponent<Control: NwgNativeCommonControl> {
    //data: Rc<RefCell<Option<NCCData<Control>>>>,
    data: Rc<RefCell<Option<Rc<NCCData<Control>>>>>,
    node: Option<NwgControlNode>,
    props: NativeCommonComponent<Control>,
}

impl<Control: NwgNativeCommonControl> NativeCommonComponentComponent<Control> {
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
                on_event: self.props.on_event.clone(),
                build: self.props.build.clone(),
            })));
            self.node = Some(node.clone());
            node
        }
    }
}

impl<Control: NwgNativeCommonControl> StateFunction for NativeCommonComponentComponent<Control> {
    type Props = NativeCommonComponent<Control>;
    type Output = NwgControlNode;
    fn build(props: Self::Props) -> (Self::Output, Self) {
        let mut component = Self {
            data: Rc::new(RefCell::new(None)),
            node: None,
            props: props,
        };

        let node = component.get_node();

        (node, component)
    }
    fn changed(&mut self, props: Self::Props) -> Self::Output {
        // TODO check if props changed, especially on_event
        self.props = props;
        self.get_node()
    }
}

pub struct NativeCommonComponentNode<Control: NwgNativeCommonControl> {
    data: Rc<RefCell<Option<Rc<NCCData<Control>>>>>,
    on_event: Rc<dyn Fn(&nwg::Event, &nwg::EventData, &nwg::ControlHandle)>, // TODO double Rc
    build: Rc<dyn Fn(&nwg::ControlHandle) -> Control>,
}

impl<Control: NwgNativeCommonControl> NwgControlNodeTrait for NativeCommonComponentNode<Control> {
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
            let control = (self.build)(parent_handle);

            let handler = {
                let on_event = self.on_event.clone();
                let control_handle = control.handle().clone();
                nwg::bind_event_handler(
                    &control.handle(),
                    parent_handle,
                    move |event, event_data, handle| {
                        if handle == control_handle {
                            (on_event)(&event, &event_data, &handle);
                        }
                    }
                )
            };

            let data = NCCData {
                parent_handle: parent_handle.clone(),
                component: control,
                handler: NwgHandler(handler),
            };

            let data = Rc::new(data);
            *self.data.borrow_mut() = Some(data.clone());
            data
        };

        data.component.handle().clone()
    }
}

impl<Control: NwgNativeCommonControl> NwgControlRefData for NCCData<Control> {
    fn native(&self) -> &dyn NwgNativeCommonControl {
        &self.component
    }
}

trait NwgControlRefData {
    #[must_use]
    fn native(&self) -> &dyn NwgNativeCommonControl;
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