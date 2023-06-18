
use std::{rc::{Rc, Weak}, cell::RefCell};

use native_windows_gui as nwg;

use crate::component::Component;

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
    data: Rc<RefCell<Weak<NCCData<Control>>>>,
    node: Option<NwgControlNode>,
    props: NativeCommonComponent<Control>,
}

impl<Control: NwgNativeCommonControl> NativeCommonComponentComponent<Control> {
    pub fn if_control<F: FnOnce(&Control)>(&self, f: F) {
        if let Some(data) = self.data.borrow().upgrade() {
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

impl<Control: NwgNativeCommonControl> Component for NativeCommonComponentComponent<Control> {
    type Props = NativeCommonComponent<Control>;
    type Output = NwgControlNode;
    fn build(props: Self::Props) -> (Self::Output, Self) {
        let mut component = Self {
            data: Rc::new(RefCell::new(Weak::new())),
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
    data: Rc<RefCell<Weak<NCCData<Control>>>>,
    on_event: Rc<dyn Fn(&nwg::Event, &nwg::EventData, &nwg::ControlHandle)>, // TODO double Rc
    build: Rc<dyn Fn(&nwg::ControlHandle) -> Control>,
}

impl<Control: NwgNativeCommonControl> NwgControlNodeTrait for NativeCommonComponentNode<Control> {
    fn handle_from_parent<'parent>(&mut self, parent: &'parent nwg::ControlHandle) -> NwgControlHandleRef<'parent> {
        let data = {
            let data = self.data.borrow();
            if let Some(current_data) = data.upgrade() {
                if current_data.parent_handle == *parent {
                    Some(current_data)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let data = if let Some(data) = data {
            data
        } else {
            let control = (self.build)(parent);

            let handler = {
                let on_event = self.on_event.clone();
                let control_handle = control.handle().clone();
                nwg::bind_event_handler(
                    &control.handle(),
                    parent,
                    move |event, event_data, handle| {
                        if handle == control_handle {
                            (on_event)(&event, &event_data, &handle);
                        }
                    }
                )
            };

            let data = NCCData {
                parent_handle: parent.clone(),
                component: control,
                handler: NwgHandler(handler),
            };

            let data = Rc::new(data);
            *self.data.borrow_mut() = Rc::downgrade(&data);
            data
        };

        NwgControlHandleRef::new(parent, data)
    }
}

impl<Control: NwgNativeCommonControl> NwgControlRefData for NCCData<Control> {
    fn native(&self) -> &dyn NwgNativeCommonControl {
        &self.component
    }
}

trait NwgControlRefData {
    fn native(&self) -> &dyn NwgNativeCommonControl;
}

#[derive(Clone)]
pub struct NwgControlHandleRef<'parent> {
    parent: &'parent nwg::ControlHandle,
    //_phantom: std::marker::PhantomData<&'parent ()>,
    data: Rc<dyn NwgControlRefData>,
}

impl<'parent> NwgControlHandleRef<'parent> {
    fn new(parent: &'parent nwg::ControlHandle, data: Rc<dyn NwgControlRefData>) -> Self {
        Self {
            parent,
            //_phantom: std::marker::PhantomData,
            data,
        }
    }

    pub fn parent_handle(&self) -> &'parent nwg::ControlHandle {
        self.parent
    }

    pub fn handle(&self) -> &nwg::ControlHandle {
        self.data.native().handle()
    }
}

pub trait NwgControlNodeTrait {
    /// Given the parent, privides an object that "owns" the native control through a [`Rc`].
    ///
    /// The parent must outlive the returned object.
    fn handle_from_parent<'parent>(&mut self, parent: &'parent nwg::ControlHandle) -> NwgControlHandleRef<'parent>;
}

#[derive(Clone)]
pub struct NwgControlNode(pub Rc<RefCell<dyn NwgControlNodeTrait>>);

impl PartialEq for NwgControlNode {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}