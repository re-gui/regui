use std::{rc::{Rc, Weak}, cell::RefCell};

use native_windows_gui as nwg;

use crate::{context::Context, functional_component::UiBuilder, component::{ComponentProps, Component}};

use self::components::{WindowingStateProps, WindowingComponent};

pub mod components;

struct ApplicationInner {
}

impl ApplicationInner {
    fn new() -> Self {
        // TODO ...
        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

        Self {
        }
    }
}

impl Drop for ApplicationInner {
    fn drop(&mut self) {
        // TODO
    }
}

pub struct Application {
    inner: Rc<RefCell<ApplicationInner>>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(ApplicationInner::new())),
        }
    }
    pub fn ctx(&self) -> NwgCtx {
        NwgCtx {
            app_inner: Rc::downgrade(&self.inner),
        }
    }
}

#[derive(Clone)]
pub struct NwgCtx {
    app_inner: Weak<RefCell<ApplicationInner>>,
}

//impl NwgCommonControl for nwg::Window {
//    fn handle(&self) -> &nwg::ControlHandle {
//        &self.handle
//    }
//}

impl NwgCtx {
    //pub fn window(&self) -> &nwg::Window {
    //    &self.window
    //}
    pub fn dispatch(&self) {
        nwg::dispatch_thread_events();
    }
    pub fn run_ui<UiProps: WindowingStateProps>(&self, props: UiProps) {
        let _windowing = WindowingComponent::build(self, props);
        self.dispatch();
    }
}

impl Context for NwgCtx {
}

/// A component that is a child of a window, might not be a window itself.
///
/// It could be, for example, a list of widgets.
pub trait NwgChildComponent {
    fn set_parent_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx);
}

impl NwgChildComponent for () {
    fn set_parent_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) {
    }
}


pub trait WrapIntoNwgChildComponent {
    fn wrap(self) -> Rc<RefCell<dyn NwgChildComponent>>;
}

impl<T: NwgChildComponent + 'static> WrapIntoNwgChildComponent for T {
    fn wrap(self) -> Rc<RefCell<dyn NwgChildComponent>> {
        Rc::new(RefCell::new(self))
    }
}

impl NwgChildComponent for Vec<Rc<RefCell<dyn NwgChildComponent>>> {
    fn set_parent_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) {
        for child in self {
            child.borrow_mut().set_parent_handle(parent_window, ctx);
        }
    }
}

pub struct ChildListBuilder<'builder>(&'builder UiBuilder<NwgCtx>, Vec<Rc<RefCell<dyn NwgChildComponent>>>);

impl<'builder> ChildListBuilder<'builder> {
    pub fn new(builder: &'builder UiBuilder<NwgCtx>) -> Self {
        Self(builder, Vec::new())
    }
    pub fn build(self) -> Vec<Rc<RefCell<dyn NwgChildComponent>>> {
        self.1
    }
    pub fn with<Props, Child: 'static>(mut self, props: Props) -> Self // TODO remove static
    where
        Props: ComponentProps<NwgCtx, AssociatedComponent = Child>,
        Child: NwgChildComponent,
    {
        self.add(props);
        self
    }
    pub fn add<Props, Child: 'static>(&mut self, props: Props) // TODO remove static
    where
        Props: ComponentProps<NwgCtx, AssociatedComponent = Child>,
        Child: NwgChildComponent,
    {
        self.1.push(self.0.get(props));
    }
}

/// Component that is also a window
//pub trait NwgChildWindowComponent {
pub trait NwgWidget: NwgChildComponent { // TODO consider removing NwgChildComponent requirement and provide default impl
    fn set_parent_and_get_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) -> &nwg::ControlHandle;
    fn current_handle(&self) -> Option<&nwg::ControlHandle>;
}

//impl<T: NwgChildComponent + NwgControl> NwgChildWidget for T {}