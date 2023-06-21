
use std::{rc::Rc, cell::RefCell};

use native_windows_gui as nwg;
use regui::StateFunction;

use crate::{WithNwgControlHandle, ControlEvent, NwgNode, NwgNodeTrait};

pub struct NativeCommonComponentProps<Control: WithNwgControlHandle> {
    pub build: Rc<dyn Fn(&nwg::ControlHandle) -> Control>,
    pub on_native_event: Rc<dyn Fn(&nwg::Event, &nwg::EventData, &nwg::ControlHandle, &Control)>,
    pub on_event: Rc<dyn Fn(&ControlEvent)>,
}

pub struct NativeCommonComponent<Control: WithNwgControlHandle> {
    data: Rc<RefCell<Option<Rc<NCCData<Control>>>>>,
    node: Option<NwgNode<nwg::ControlHandle>>,
    props: NativeCommonComponentProps<Control>,
}

impl<Control: WithNwgControlHandle> NativeCommonComponent<Control> {
    pub fn if_control<F: FnOnce(&Control)>(&self, f: F) {
        if let Some(data) = self.data.borrow().as_ref() {
            f(&data.component);
        }
    }
    pub fn get_node(&mut self) -> NwgNode<nwg::ControlHandle> {
        if let Some(node) = self.node.clone() {
            node
        } else {
            let node = NwgNode::<nwg::ControlHandle>(Rc::new(RefCell::new(NativeCommonComponentNode {
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

impl<Control: WithNwgControlHandle> StateFunction for NativeCommonComponent<Control> {
    type Input = NativeCommonComponentProps<Control>;
    type Output = NwgNode<nwg::ControlHandle>;
    fn build(props: Self::Input) -> (Self::Output, Self) {
        let mut component = Self {
            data: Rc::new(RefCell::new(None)),
            node: None,
            props,
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

impl<Control: WithNwgControlHandle> NwgNodeTrait for NativeCommonComponentNode<Control> {
    type Output = nwg::ControlHandle;
    fn from_parent(&mut self, parent_handle: &nwg::ControlHandle) -> Self::Output {
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

                                let event = ControlEvent::from_nwg_event(&event, &event_data, &handle, control.as_ref());
                                if let Some(event) = event {
                                    (on_event)(&event);
                                }
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

struct NCCData<Control: WithNwgControlHandle> {
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