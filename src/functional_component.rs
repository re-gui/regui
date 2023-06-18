use std::{rc::{Rc, Weak}, any::Any, cell::RefCell};

use crate::state_function::{StateFunctionProps, StateFunction, LiveValue, LiveLink, LiveValueEmitter};

pub struct StateManagerInner<State> {
    state: RefCell<State>,
    builder_fn: RefCell<Rc<dyn Fn(StateLink<State>)>>, // TODO maybe Rc is not needed
    to_rerun: RefCell<bool>,
    in_run: RefCell<bool>,
}

impl<State> StateManagerInner<State> {
    fn run(&self, self_link: StateLink<State>) {
        *self.to_rerun.borrow_mut() = true;
        if *self.in_run.borrow() {
            return;
        } else {
            *self.in_run.borrow_mut() = true;
            while *self.to_rerun.borrow() {
                *self.to_rerun.borrow_mut() = false;
                let build = self.builder_fn.borrow().clone();
                build(self_link.clone());
            }
            *self.in_run.borrow_mut() = false;
        }
    }

    fn on_state<R>(&self, on_state: impl FnOnce(&State) -> R) -> R {
        let state = self.state.borrow();
        on_state(&state)
    }

    fn on_mut_state<R>(&self, on_state: impl FnOnce(&mut State) -> R) -> R {
        let mut state = self.state.borrow_mut();
        on_state(&mut state)
    }
}

pub struct StateManager<State> {
    inner: Rc<StateManagerInner<State>>,
}

impl<State: 'static> StateManager<State> {
    pub fn new(state: State) -> Self {
        Self {
            inner: Rc::new(StateManagerInner {
                state: RefCell::new(state),
                builder_fn: RefCell::new(Rc::new(|_| {})),
                to_rerun: RefCell::new(false),
                in_run: RefCell::new(false),
            }),
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

    pub fn set_builder(&self, builder: impl Fn(StateLink<State>) + 'static) {
        *self.inner.builder_fn.borrow_mut() = Rc::new(builder);
    }

    pub fn link(&self) -> StateLink<State> {
        StateLink {
            state: Rc::downgrade(&self.inner),
        }
    }

    //pub fn take_state(&self) -> Option<State> {
    //    // TODO to check
    //    let mut manager = self.inner.borrow_mut();
    //    manager.state.take()
    //}

    //pub fn set_state(&self, state: State) {
    //    self.inner.borrow_mut().state = Some(state);
    //    self.inner.borrow().run(self.link()); // TODO run???
    //}

    pub fn on_state<R>(&self, on_state: impl FnOnce(&State) -> R) -> R {
        self.inner.on_state(on_state)
    }

    pub fn on_mut_state<R>(&self, on_state: impl FnOnce(&mut State) -> R) -> R {
        self.inner.on_mut_state(on_state)
    }
}

pub struct StateLink<P> {
    state: Weak<StateManagerInner<P>>,
}

impl<P> Clone for StateLink<P> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<State> StateLink<State> {

    //pub fn set(&self, state: P) {
    //    if let Some(manager) = self.state.upgrade() {
    //        manager.borrow_mut().state = Some(state);
    //        manager.borrow().run(self.clone());
    //    }
    //    // if expired, no effect
    //}

    pub fn update(&self, update: impl FnOnce(&mut State)) {
        // TODO call self.set, instead of duplicating the code
        if let Some(manager) = self.state.upgrade() {
            let _r = manager.on_mut_state(|state| {
                update(state);
            });
            manager.run(self.clone());
        }
        // if expired, no effect
    }

    // TODO update_eq

    //fn upgrade(&self) -> Option<StateManagerWrapper<P, F>> {
    //    self.state.upgrade().map(|state| StateManagerWrapper { state })
    //}
}

pub struct FunctionsCacheData {
    functions: Vec<Rc<dyn Any>>,
    functions_pos: usize,
}

pub struct FunctionsCache {
    data: RefCell<FunctionsCacheData>,
    live_link: Rc<RefCell<LiveLink>>,
}

impl FunctionsCache {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(FunctionsCacheData {
                functions: Vec::new(),
                functions_pos: 0,
            }),
            live_link: Rc::new(RefCell::new(LiveLink::new())),
        }
    }

    // TODO ???
    pub fn emitter(&self) -> LiveValueEmitter {
        let (_, emitter) = self.live_link.borrow_mut().make_live_value(()).into_tuple();
        emitter
    }

    pub fn eval_live<Props: StateFunctionProps, T>(&self, props: Props) -> T
    where
        Props::AssociatedComponent: StateFunction<Output = LiveValue<T>>,
    {
        let (value, emitter) = self.eval_state_function::<Props::AssociatedComponent>(props).into_tuple();
        emitter.listen({
            let live_link = self.live_link.clone();
            move || {
                live_link.borrow_mut().tell_update();
            }
        });
        value
    }

    pub fn eval<Props: StateFunctionProps>(&self, props: Props) -> <Props::AssociatedComponent as StateFunction>::Output {
        self.eval_state_function::<Props::AssociatedComponent>(props)
    }

    // TODO get_if_new and get_if_changed

    pub fn eval_state_function<SF: StateFunction>(&self, props: SF::Props) -> SF::Output {
        let mut data = self.data.borrow_mut();
        let pos = data.functions_pos;
        let result = if pos < data.functions.len() {
            let function = data.functions[pos].clone().downcast::<RefCell<SF>>();
            let function = if let Ok(function) = function {
                if function.borrow().reuse_with(&props) {
                    function.borrow_mut().changed(props)
                } else { // TODO avoid this IF nesting and else block code repetition
                    let (result, component) = SF::build(props);
                    let component = Rc::new(RefCell::new(component));
                    data.functions[pos] = component.clone();
                    result
                }
            } else {
                let (result, component) = SF::build(props);
                let function = Rc::new(RefCell::new(component));
                data.functions.insert(pos, function.clone());
                result
            };
            data.functions_pos += 1;
            function
        } else {
            let (result, function) = SF::build(props);
            let function = Rc::new(RefCell::new(function));
            data.functions.push(function.clone());
            data.functions_pos = data.functions.len();
            result
        };

        result
    }

    // TODO hide to the user using another struct
    pub fn finish(&mut self) {
        let mut data = self.data.borrow_mut();
        let pos = data.functions_pos;
        data.functions.truncate(pos);
        data.functions_pos = 0;
    }
}