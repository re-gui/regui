use std::{rc::Rc, cell::RefCell};

use native_windows_gui as nwg;

use crate::{Callback, component::Component, functional_component::{UiBuilder, StateLink, StateManager}};

use super::{NwgCtx, NwgChildComponent};

mod button; pub use button::*;
mod label; pub use label::*;
mod grid_layout; pub use grid_layout::*;
mod window; pub use window::*;
mod text_input; pub use text_input::*;

/// A [`native_windows_gui`] [Common Control](https://learn.microsoft.com/en-us/windows/win32/controls/common-controls-intro)
pub trait NwgNativeCommonControl: 'static {
    fn handle(&self) -> &nwg::ControlHandle;
}

//pub trait NwgControl: 'static {
//    fn handle(&self) -> Option<&nwg::ControlHandle>;
//}

pub struct NativeCommonComponent<Control: NwgNativeCommonControl> {
    component: Rc<Control>,
    handler: Option<nwg::EventHandler>,
    window_handle: nwg::ControlHandle,
    old_props: Option<NativeCommonComponentProperties<Control>>,
}

impl<Control: NwgNativeCommonControl> NativeCommonComponent<Control> {
    pub fn build_native(
        window_handle: nwg::ControlHandle,
        build: impl FnOnce(nwg::ControlHandle) -> Control,
        handler: Option<impl Fn(nwg::Event, nwg::EventData, nwg::ControlHandle) + 'static>,
    ) -> Self {
        let component = build(window_handle);

        let component = Rc::new(component);

        let handler = {
            let events_component = Rc::downgrade(&component);

            if let Some(handler) = handler {
                Some(nwg::bind_event_handler(
                    &component.handle(),
                    &window_handle,
                    move |event, event_data, handle| {
                        if let Some(events_component) = events_component.upgrade() {
                            if handle == *events_component.handle() {
                                handler(event, event_data, handle);
                            }
                        }
                    }
                ))
            } else {
                None
            }
        };

        Self {
            component,
            handler,
            window_handle,
            old_props: None,
        }
    }

    pub fn change_handler(
        &mut self,
        handler: Option<impl Fn(nwg::Event, nwg::EventData, nwg::ControlHandle) + 'static>,
    ) {
        if let Some(handler) = self.handler.take() {
            nwg::unbind_event_handler(&handler);
        }

        let handler = match handler {
            Some(handler) => handler,
            None => return,
        };
        
        let events_component = Rc::downgrade(&self.component);
        self.handler = Some(nwg::bind_event_handler(
            &self.component.handle(),
            &self.window_handle,
            move |event, event_data, handle| {
                if let Some(events_component) = events_component.upgrade() {
                    if handle == *events_component.handle() {
                        handler(event, event_data, handle);
                    }
                }
            }
        ));
    }
}

impl<Control: NwgNativeCommonControl> Drop for NativeCommonComponent<Control> {
    fn drop(&mut self) {
        if let Some(handler) = self.handler.take() {
            // TODO panics, might be unnecessary here nwg::unbind_event_handler(&handler);
        }
    }
}

pub struct NativeCommonComponentProperties<Control: NwgNativeCommonControl> {
    parent_window: nwg::ControlHandle,
    build: Callback<nwg::ControlHandle, Control>,
    on_event: Option<Callback<(nwg::Event, nwg::EventData, nwg::ControlHandle)>>,
}

impl<Control: NwgNativeCommonControl> Clone for NativeCommonComponentProperties<Control> {
    fn clone(&self) -> Self {
        Self {
            parent_window: self.parent_window.clone(),
            build: self.build.clone(),
            on_event: self.on_event.clone(),
        }
    }
}

impl<Control: NwgNativeCommonControl> Component<NwgCtx> for NativeCommonComponent<Control> {
    type Props = NativeCommonComponentProperties<Control>;
    fn build(_ctx: &NwgCtx, props: Self::Props) -> Self {
        let on_event = props.on_event.clone();
        let mut s = Self::build_native(
            props.parent_window,
            |window_handle| {
                props.build.call(window_handle)
            },
            on_event.map(|on_event| move |event, event_data, handle| {
                on_event.call((event, event_data, handle))
            }),
        );
        s.old_props = Some(props); // TODO change this system
        s
    }

    fn changed(&mut self, props: Self::Props, ctx: &NwgCtx) {
        let old_props = self.old_props.as_ref().unwrap();
        if old_props.build != props.build {
            *self = Self::build(ctx, props.clone());
        } else {
            if old_props.on_event != props.on_event {
                let on_event = props.on_event.clone();
                self.change_handler(on_event.map(|on_event| move |event, event_data, handle| {
                    on_event.call((event, event_data, handle))
                }));
            }
        }
        self.old_props = Some(props); // TODO change this system
    }
}

pub trait WindowingStateProps: 'static { // TODO remove 'static
    type State;
    fn build_state(self, old_state: Option<Self::State>) -> Self::State;
    fn build_ui(builder: &UiBuilder<NwgCtx>, state: &Self::State, link: StateLink<Self::State, NwgCtx>);
}

pub struct WindowingComponent<Props: WindowingStateProps> {
    state_manager: StateManager<Props::State, NwgCtx>,
}

impl<Props: WindowingStateProps + 'static> Component<NwgCtx> for WindowingComponent<Props> {
    type Props = Props;
    fn build(ctx: &NwgCtx, props: Self::Props) -> Self {
        let state_manager = StateManager::new_with(
            ctx.clone(),
            props.build_state(None)
        );

        let ui_builder = Rc::new(RefCell::new(UiBuilder::new(ctx.clone())));

        state_manager.set_builder({
            let ui_builder = ui_builder.clone();
            let ctx = ctx.clone();
            move |state, link| {
                let mut builder = ui_builder.borrow_mut();
                let ctx = ctx.clone();
                let r = Props::build_ui(&builder, state, link);
                builder.finish();
            }
        });

        Self {
            state_manager,
        }
    }
    fn changed(&mut self, props: Self::Props, ctx: &NwgCtx) {
        // TODO use update_eq if possible?
        self.state_manager.link().update(|s| *s = props.build_state(self.state_manager.take_state()));
    }
    // TODO fn reuse_with(&self, _props: &Self::Props) -> bool;
}

pub trait StateProps: 'static { // TODO remove 'static
    type State;
    //type Out: ComponentProps<NwgCtx> where <Self::Out as ComponentProps<NwgCtx>>::AssociatedComponent: Sized;
    type Out: ?Sized;
    fn build_state(self, old_state: Option<Self::State>) -> Self::State;
    fn build_ui(builder: &UiBuilder<NwgCtx>, state: &Self::State, link: StateLink<Self::State, NwgCtx>) -> Self::Out;
}

// TODO maybe bool to chech when to stop?

pub struct StateChildComponent<Props: StateProps> {
    state_manager: StateManager<Props::State, NwgCtx>,
    parent: Rc<RefCell<Option<nwg::ControlHandle>>>,
    component: Rc<RefCell<Option<Rc<RefCell<dyn NwgChildComponent>>>>>,
    //ui_builder: Rc<RefCell<UiBuilder<NwgCtx>>>,
}

impl<Props: StateProps<Out = Rc<RefCell<dyn NwgChildComponent>>> + 'static> Component<NwgCtx> for StateChildComponent<Props> {
    type Props = Props;
    fn build(ctx: &NwgCtx, props: Self::Props) -> Self {
        let state_manager = StateManager::new_with(
            ctx.clone(),
            props.build_state(None)
        );

        let parent: Rc<RefCell<Option<nwg::ControlHandle>>> = Rc::new(RefCell::new(None));
        let component = Rc::new(RefCell::new(None));

        let ui_builder = Rc::new(RefCell::new(UiBuilder::new(ctx.clone())));

        state_manager.set_builder({
            let parent = parent.clone();
            let component = component.clone();
            let ctx = ctx.clone();
            let builder = ui_builder.clone();
            move |state, link| {
                let mut builder = builder.borrow_mut();
                let result_component = Props::build_ui(&builder, state, link);
                builder.finish();
                let mut old_component = component.borrow_mut();
                let changed = if let Some(old_component) = old_component.as_ref() {
                    !Rc::ptr_eq(&result_component, old_component)
                } else {
                    true
                };
                if changed {
                    // destroy old component
                    {
                        if let Some(old_component) = old_component.as_ref() {
                            assert_eq!(Rc::strong_count(old_component), 1);
                        }
                        old_component.take();
                    }
                    {
                        let mut component = result_component.borrow_mut();
                        if parent.borrow().is_some() {
                            component.set_parent_handle(parent.borrow().as_ref().unwrap().clone(), &ctx);
                        }
                    }
                    old_component.replace(result_component);
                }
            }
        });

        Self {
            state_manager,
            parent,
            component,
            //ui_builder,
        }
    }
    fn changed(&mut self, props: Self::Props, _ctx: &NwgCtx) {
        // TODO use update_eq if possible?
        let old_state = self.state_manager.take_state();
        self.state_manager.link().set(props.build_state(old_state));
    }
    // TODO fn reuse_with(&self, _props: &Self::Props) -> bool {
    // TODO     todo!()
    // TODO }
}

impl<Props: StateProps + 'static> NwgChildComponent for StateChildComponent<Props> {
    fn set_parent_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) {
        let mut old_parent = self.parent.borrow_mut();
        if let Some(old_parent) = &*old_parent {
            if old_parent == &parent_window {
                return;
            }
        }
        *old_parent = Some(parent_window.clone());
        let component = self.component.borrow();
        if let Some(component) = component.as_ref() {
            let mut component = component.borrow_mut();
            component.set_parent_handle(parent_window, ctx);
        }
    }
}