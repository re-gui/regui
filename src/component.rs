use std::{rc::{Rc, Weak}, any::Any, cell::RefCell, collections::VecDeque};

use crate::state_function::{StateFunctionProps, StateFunction, LiveValue, LiveLink, LiveValueEmitter};

pub struct StateManagerInner<State> {
    state: RefCell<State>,
    //builder_fn: RefCell<Option<Rc<dyn Fn(&State, StateLink<State>)>>>, // TODO maybe Rc is not needed
    builder_fn: RefCell<Rc<dyn Fn(&State, StateLink<State>)>>, // TODO maybe Rc is not needed
    message_queue: RefCell<VecDeque<Box<dyn FnOnce(&mut State)>>>,
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
                //if let Some(build) = build {
                    build(&self.state.borrow(), self_link.clone());
                //}
            }
            *self.in_run.borrow_mut() = false;
        }
    }

    fn push_on_queue(&self, message: impl FnOnce(&mut State) + 'static) {
        const MAX_QUEUE_SIZE: usize = 1000;
        if self.message_queue.borrow().len() > MAX_QUEUE_SIZE {
            panic!("Message queue is too big, maybe you have infinite update loop?");
        }
        self.message_queue.borrow_mut().push_back(Box::new(message));
    }

    fn run_queue(&self, self_link: StateLink<State>) {
        if self.message_queue.borrow().is_empty() {
            return;
        }

        let runned = if let Ok(mut state) = self.state.try_borrow_mut() {
            let pick = || self.message_queue.borrow_mut().pop_front();
            while let Some(message) = pick() {
                message(&mut state);
            }
            true
        } else {
            false
        };
        if runned {
            self.run(self_link);
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
                //builder_fn: RefCell::new(None),
                builder_fn: RefCell::new(Rc::new(|_, _| {})),
                message_queue: RefCell::new(VecDeque::new()),
                to_rerun: RefCell::new(false),
                in_run: RefCell::new(false),
            }),
        }
    }

    pub fn set_builder(&self, builder: impl Fn(&State, StateLink<State>) + 'static) {
        *self.inner.builder_fn.borrow_mut() = Rc::new(builder);
        self.inner.run(self.link());
    }

    pub fn link(&self) -> StateLink<State> {
        StateLink {
            state: Rc::downgrade(&self.inner),
        }
    }

    pub fn on_state<R>(&self, on_state: impl FnOnce(&State) -> R) -> R {
        let r = self.inner.on_state(on_state);
        self.inner.run_queue(self.link());
        r
    }

    pub fn on_mut_state<R>(&self, on_state: impl FnOnce(&mut State) -> R) -> R {
        let r = self.inner.on_mut_state(on_state);
        self.inner.run_queue(self.link());
        r
    }
}

pub struct StateLink<P> {
    state: Weak<StateManagerInner<P>>,
}
// TODO An intermediate between this and an mpsc

impl<P> Clone for StateLink<P> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<State> StateLink<State> {
    /// Sends an update to the state.
    pub fn send_update(&self, update: impl FnOnce(&mut State) + 'static) {
        // TODO call self.set, instead of duplicating the code
        if let Some(manager) = self.state.upgrade() {
            manager.push_on_queue(update);
            manager.run_queue(self.clone());
        }
        // if expired, no effect
    }

    // TODO update_eq
}

impl<State> StateLink<State>
where
    State: Component,
{
    /// Sends a message to the state.
    ///
    /// The behaviour of this function is the same as [`send_update`](StateLink::send_update), but it is more semantic.
    ///
    /// # Notes
    /// See the source of this function for more details.
    pub fn send_message(&self, message: State::Message) {
        self.send_update(|state| {
            state.on_message(message);
        });
    }
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
    #[must_use]
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
    #[must_use]
    pub fn emitter(&self) -> LiveValueEmitter {
        let (_, emitter) = self.live_link.borrow_mut().make_live_value(()).into_tuple();
        emitter
    }

    /// Same as [`eval`](FunctionsCache::eval) but unwraps the live value.
    ///
    /// This function will call [`eval`](FunctionsCache::eval) and unwrap the result.
    /// The live value emitter will be consumed: the value changes can be listened using [`emitter`](FunctionsCache::emitter).
    #[must_use]
    pub fn live<Props: StateFunctionProps, T>(&self, props: Props) -> T
    where
        Props::AssociatedFunction: StateFunction<Output = LiveValue<T>>,
    {
        self.eval_live_state_function::<Props::AssociatedFunction, T>(props)
    }

    /// Evaluates a state function from the components.
    #[must_use]
    pub fn eval<Props: StateFunctionProps>(&self, props: Props) -> <Props::AssociatedFunction as StateFunction>::Output {
        self.eval_state_function::<Props::AssociatedFunction>(props)
    }

    // TODO get_if_new and get_if_changed

    #[must_use]
    pub fn eval_live_state_function<SF, T>(&self, props: SF::Input) -> T
    where
        SF: StateFunction<Output = LiveValue<T>>,
    {
        let (value, emitter) = self.eval_state_function::<SF>(props).into_tuple();
        emitter.listen({
            let live_link = self.live_link.clone();
            move || {
                live_link.borrow_mut().tell_update();
            }
        });
        value
    }

    #[must_use]
    pub fn eval_state_function<SF: StateFunction>(&self, props: SF::Input) -> SF::Output {
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

pub trait Component: Sized + 'static { // TODO remove 'static
    type Props;
    type Out: PartialEq + Clone + 'static;
    type Message;
    #[must_use]
    fn build(props: Self::Props) -> Self; // TODO maybe link avaliable here
    fn update(&mut self, _props: Self::Props) {} // TODO maybe link avaliable here

    /// Called when a message is sent to the component.
    ///
    /// Note that it is not necessary to use this method to update the component state:
    /// you could use [`StateLink::send_update`] instead.
    ///
    /// [`StateLink::send_update`]: StateLink::send_update
    fn on_message(&mut self, _message: Self::Message) {} // TODO maybe link avaliable here
    #[must_use]
    fn view(&self, link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out;
}

pub struct LiveStateComponent<SC: Component> {
    state_manager: Rc<RefCell<StateManager<SC>>>,
    components_cache: Rc<RefCell<FunctionsCache>>,
    out: Rc<RefCell<SC::Out>>,
    live_link: LiveLink,
}

impl<SC: Component> StateFunction for LiveStateComponent<SC> {
    type Input = SC::Props;
    type Output = LiveValue<SC::Out>;
    fn build(props: Self::Input) -> (Self::Output, Self) {

        let component = SC::build(props);
        let state_manager = StateManager::<SC>::new(component);
        let components_cache = Rc::new(RefCell::new(FunctionsCache::new()));

        let result = {
            let result = state_manager.on_state(|component| {
                component.view(state_manager.link(), &components_cache.borrow_mut())
            });
            components_cache.borrow_mut().finish();
            result
        };
        let out = Rc::new(RefCell::new(result.clone()));

        let live_link = LiveLink::new();

        components_cache.borrow_mut().emitter().listen({
            let link = state_manager.link();
            move || {
                link.send_update(|_| {});
            }
        });

        let state_manager = Rc::new(RefCell::new(state_manager));

        state_manager.borrow_mut().set_builder({
            let cache = Rc::downgrade(&components_cache);
            let out = Rc::downgrade(&out);
            let live_link = live_link.clone();
            //let state_manager = state_manager.clone();
            move |component, link| {
                let cache = match cache.upgrade() {
                    Some(cache) => cache,
                    None => return,
                };
                let mut cache = cache.borrow_mut();
                let new_result = {
                    let result = component.view(link.clone(), &cache);
                    cache.finish();
                    result
                };
                let out = match out.upgrade() {
                    Some(out) => out,
                    None => return,
                };
                if new_result != *out.borrow() {
                    // set new result
                    *out.borrow_mut() = new_result.clone();
                    // signal change
                    live_link.tell_update();
                }
            }
        });

        (
            live_link.make_live_value(result),
            Self {
                state_manager,
                components_cache,
                out,
                live_link,
            }
        )
    }
    fn changed(&mut self, props: Self::Input) -> Self::Output {
        //self.state_manager.borrow().on_mut_state(|component| {
        //    component.update(props);
        //});
        let link = self.state_manager.borrow().link();
        link.send_update(|component| {
            component.update(props);
        });

        // TODO when a component internally updates its state, it will use this update function.
        // this will cause the parent's view function to be called, which will call the child's changed function (this function).
        // But this function will call update again here, wich will cause the view function to be called again.
        // This will cause the view function to be called twice, which is not good.
        // Maybe just removing this call will be enough, since the the component's update will already call the link update function if needed,
        // but this should be analyzed more in depth.
        self.state_manager.borrow().link().send_update(|_| {});

        self.live_link.make_live_value(self.out.borrow().clone())
    }
}