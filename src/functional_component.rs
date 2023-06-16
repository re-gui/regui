use std::{any::Any, rc::{Rc, Weak}, cell::RefCell, ops::Deref};

use crate::component::{Component, ComponentProps};


pub struct StateManagerInner<P, Ctx> {
    state: Option<P>,
    builder_fn: Option<Box<dyn FnMut(&P/*, &UiBuilder<Ctx>*/, StateLink<P, Ctx>)>>,
    // ! ui_builder: UiBuilder<Ctx>,
    in_build: bool,
}

impl<P, Ctx> StateManagerInner<P, Ctx> {
    fn run(&mut self, self_link: StateLink<P, Ctx>) {
        if let Some(build) = &mut self.builder_fn {
            self.in_build = true;
            build(
                self.state.as_ref().expect("state is not set, cannot update before state is set"),
                // ! &mut self.ui_builder,
                self_link, // TODO enforce at compile time that this is a link to self, maybe moving the run method to the link
            );
            self.in_build = false;
        }
        // ! self.ui_builder.reset_state_counters();
    }
}

pub struct StateManager<P, Ctx> {
    inner: Rc<RefCell<StateManagerInner<P, Ctx>>>,
}

impl<P: 'static, Ctx> StateManager<P, Ctx> {
    pub fn new(ctx: Ctx) -> Self {
        Self {
            inner: Rc::new(RefCell::new(StateManagerInner {
                state: None,
                builder_fn: None,
                // ! ui_builder: UiBuilder::new(ctx),
                in_build: false,
            })),
        }
    }

    pub fn new_with(ctx: Ctx, props: P) -> Self {
        Self {
            inner: Rc::new(RefCell::new(StateManagerInner {
                state: Some(props),
                builder_fn: None,
                // ! ui_builder: UiBuilder::new(ctx),
                in_build: false,
            })),
        }
    }

    pub fn set_builder(&self, builder: impl FnMut(&P/*, &UiBuilder<Ctx>*/, StateLink<P, Ctx>) + 'static) {
        let mut manager = self.inner.borrow_mut();
        if manager.in_build {
            panic!("Cannot set builder while building");
        }
        manager.builder_fn = Some(Box::new(builder));
        manager.run(self.link());
    }

    pub fn link(&self) -> StateLink<P, Ctx> {
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

pub struct StateLink<P, Ctx> {
    state: Weak<RefCell<StateManagerInner<P, Ctx>>>,
}

impl<P, Ctx> Clone for StateLink<P, Ctx> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<P, Ctx> StateLink<P, Ctx> {

    pub fn set(&self, state: P) {
        if let Some(manager) = self.state.upgrade() {
            let mut manager = manager.borrow_mut();
            if manager.in_build {
                panic!("Cannot update state while building"); // TODO this is never hit because of the borrow_mut. To test this, write state_link.update(|_| {}); in a build function
            }
            manager.state = Some(state);
            manager.run(self.clone());
        }
        // if expired, no effect
    }

    pub fn update(&self, update: impl FnOnce(&mut P)) {
        // TODO call self.set, instead of duplicating the code
        if let Some(manager) = self.state.upgrade() {
            let mut manager = manager.borrow_mut();
            if manager.in_build {
                panic!("Cannot update state while building"); // TODO this is never hit because of the borrow_mut. To test this, write state_link.update(|_| {}); in a build function
            }
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

pub struct UiBuilderData {
    components: Vec<Rc<dyn Any>>,
    components_pos: usize,
}

pub struct UiBuilder<Ctx> {
    data: RefCell<UiBuilderData>,
    ctx: Ctx,
}

impl<Ctx> UiBuilder<Ctx> {
    pub fn new(ctx: Ctx) -> Self {
        Self {
            data: RefCell::new(UiBuilderData {
                components: Vec::new(),
                components_pos: 0,
            }),
            ctx,
        }
    }

    pub fn get<Props: ComponentProps<Ctx>>(&self, props: Props) -> Rc<RefCell<Props::AssociatedComponent>> {
        self.component::<Props::AssociatedComponent>(props)
    }

    // TODO get_if_new and get_if_changed

    pub fn component<C: Component<Ctx>>(&self, props: C::Props) -> Rc<RefCell<C>> {
        let mut data = self.data.borrow_mut();
        let pos = data.components_pos;
        let component = if pos < data.components.len() {
            let component = data.components[pos].clone().downcast::<RefCell<C>>();
            let component = if let Ok(component) = component {
                if component.borrow().reuse_with(&props) {
                    component.borrow_mut().changed(props, &self.ctx);
                    component.clone()
                } else { // TODO avoid this IF nesting and else block code repetition
                    let component = Rc::new(RefCell::new(C::build(&self.ctx, props)));
                    data.components[pos] = component.clone();
                    component
                }
            } else {
                let component = Rc::new(RefCell::new(C::build(&self.ctx, props)));
                data.components.insert(pos, component.clone());
                component
            };
            data.components_pos += 1;
            component
        } else {
            let component = Rc::new(RefCell::new(C::build(&self.ctx, props)));
            data.components.push(component.clone());
            data.components_pos = data.components.len();
            component
        };

        component
    }

    // TODO hide to the user using another struct
    pub fn finish(&mut self) {
        let mut data = self.data.borrow_mut();
        let pos = data.components_pos;
        data.components.truncate(pos);
        data.components_pos = 0;
    }
}