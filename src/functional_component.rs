use std::{rc::{Rc, Weak}, any::Any, cell::RefCell};

use crate::component::{ComponentProps, Component, LiveValue, LiveLink, AsLiveValue, LiveValueEmitter};

pub struct StateManagerInner<State> {
    state: Option<State>,
    builder_fn: Option<Box<dyn FnMut(&State, StateLink<State>)>>,
}

impl<State> StateManagerInner<State> {
    fn run(&mut self, self_link: StateLink<State>) {
        if let Some(build) = &mut self.builder_fn {
            build(
                self.state.as_ref().expect("state is not set, cannot update before state is set"),
                self_link, // TODO enforce at compile time that this is a link to self, maybe moving the run method to the link
            );
        }
    }
}

pub struct StateManager<P> {
    inner: Rc<RefCell<StateManagerInner<P>>>,
}

impl<P: 'static> StateManager<P> {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(StateManagerInner {
                state: None,
                builder_fn: None,
            })),
        }
    }

    pub fn new_with(props: P) -> Self {
        Self {
            inner: Rc::new(RefCell::new(StateManagerInner {
                state: Some(props),
                builder_fn: None,
            })),
        }
    }

    //pub fn set_builder_run(&self, builder: impl FnMut(&P/*, &UiBuilder<Ctx>*/, StateLink<P>) + 'static) {
    //    let mut manager = self.inner.borrow_mut();
    //    if manager.in_build {
    //        panic!("Cannot set builder while building");
    //    }
    //    manager.builder_fn = Some(Box::new(builder));
    //    manager.run(self.link());
    //}

    pub fn set_builder(&self, builder: impl FnMut(&P/*, &UiBuilder<Ctx>*/, StateLink<P>) + 'static) {
        let mut manager = self.inner.borrow_mut();
        manager.builder_fn = Some(Box::new(builder));
    }

    pub fn link(&self) -> StateLink<P> {
        StateLink {
            state: Rc::downgrade(&self.inner),
        }
    }

    pub fn take_state(&self) -> Option<P> {
        // TODO to check
        let mut manager = self.inner.borrow_mut();
        manager.state.take()
    }
}

pub struct StateLink<P> {
    state: Weak<RefCell<StateManagerInner<P>>>,
}

impl<P> Clone for StateLink<P> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<P> StateLink<P> {

    pub fn set(&self, state: P) {
        if let Some(manager) = self.state.upgrade() {
            let mut manager = manager.borrow_mut();
            manager.state = Some(state);
            manager.run(self.clone());
        }
        // if expired, no effect
    }

    pub fn update(&self, update: impl FnOnce(&mut P)) {
        // TODO call self.set, instead of duplicating the code
        if let Some(manager) = self.state.upgrade() {
            let mut manager = manager.borrow_mut();
            update(manager.state.as_mut().unwrap());
            manager.run(self.clone());
        }
        // if expired, no effect
    }

    // TODO update_eq

    //fn upgrade(&self) -> Option<StateManagerWrapper<P, F>> {
    //    self.state.upgrade().map(|state| StateManagerWrapper { state })
    //}
}

pub struct ComponentsCacheData {
    components: Vec<Rc<dyn Any>>,
    components_pos: usize,
}

pub struct ComponentsCache {
    data: RefCell<ComponentsCacheData>,
    live_link: Rc<RefCell<LiveLink>>,
}

impl ComponentsCache {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(ComponentsCacheData {
                components: Vec::new(),
                components_pos: 0,
            }),
            live_link: Rc::new(RefCell::new(LiveLink::new())),
        }
    }

    // TODO ???
    pub fn emitter(&self) -> LiveValueEmitter {
        let (_, emitter) = self.live_link.borrow_mut().make_live_value(()).into_tuple();
        emitter
    }

    pub fn get_live<Props: ComponentProps, T>(&self, props: Props) -> T
    where
        Props::AssociatedComponent: Component<Output = LiveValue<T>>,
    {
        let (value, emitter) = self.component::<Props::AssociatedComponent>(props).into_tuple();
        emitter.listen({
            let live_link = self.live_link.clone();
            move || {
                live_link.borrow_mut().tell_update();
            }
        });
        value
    }

    pub fn get<Props: ComponentProps>(&self, props: Props) -> <Props::AssociatedComponent as Component>::Output {
        self.component::<Props::AssociatedComponent>(props)
    }

    // TODO get_if_new and get_if_changed

    pub fn component<C: Component>(&self, props: C::Props) -> C::Output {
        let mut data = self.data.borrow_mut();
        let pos = data.components_pos;
        let result = if pos < data.components.len() {
            let component = data.components[pos].clone().downcast::<RefCell<C>>();
            let component = if let Ok(component) = component {
                if component.borrow().reuse_with(&props) {
                    component.borrow_mut().changed(props)
                } else { // TODO avoid this IF nesting and else block code repetition
                    let (result, component) = C::build(props);
                    let component = Rc::new(RefCell::new(component));
                    data.components[pos] = component.clone();
                    result
                }
            } else {
                let (result, component) = C::build(props);
                let component = Rc::new(RefCell::new(component));
                data.components.insert(pos, component.clone());
                result
            };
            data.components_pos += 1;
            component
        } else {
            let (result, component) = C::build(props);
            let component = Rc::new(RefCell::new(component));
            data.components.push(component.clone());
            data.components_pos = data.components.len();
            result
        };

        result
    }

    // TODO hide to the user using another struct
    pub fn finish(&mut self) {
        let mut data = self.data.borrow_mut();
        let pos = data.components_pos;
        data.components.truncate(pos);
        data.components_pos = 0;
    }
}