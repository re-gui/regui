use std::{rc::Rc, ops::Deref, fmt::Debug, any::Any, cell::RefCell};

use crate::component::{FunctionsCache, Component, StateLink};

/// Declares a function component
///
/// # Example
/// ```ignore
/// function_component!(pub MyComponent my_component(i32) -> Vec<NwgControlNode>);
/// fn my_component(props: &i32, cache: &FunctionsCache, state: &mut State) -> Vec<NwgControlNode> {
///     // ...
/// }
/// ```
///
/// Note that the following are valid:
/// - `function_component!(pub MyComponent ...)` declares a public component
/// - `function_component!(priv MyComponent ...)` declares a private component
/// - `function_component!(MyComponent ...)` also declares a private component
#[macro_export]
macro_rules! function_component {
    (pub $name:ident $func_name:ident ($props:ty) -> $out:ty) => {
        pub struct $name;
        impl $crate::function_component::FCFunction for $name {
            type Props = $props;
            type Out = $out;
            fn call<'a>(props: &Self::Props, cache: &$crate::component::FunctionsCache, state: &mut State<'a>) -> Self::Out {
                $func_name(props, cache, state)
            }
        }
        impl $crate::component::EvalFromCache for $name {
            type Input = $props;
            type Out = $out;
            fn eval(cache: &FunctionsCache, input: Self::Input) -> Self::Out {
                cache.eval_live::<LiveStateComponent<FunctionalComponent<$name>>, $out>(input)
            }
        }
    };
    (priv $name:ident $func_name:ident ($props:ty) -> $out:ty) => {
        struct $name;
        impl $crate::function_component::FCFunction for $name {
            type Props = $props;
            type Out = $out;
            fn call<'a>(props: &Self::Props, cache: &$crate::component::FunctionsCache, state: &mut State<'a>) -> Self::Out {
                $func_name(props, cache, state)
            }
        }
        impl $crate::component::EvalFromCache for $name {
            type Input = $props;
            type Out = $out;
            fn eval(cache: &FunctionsCache, input: Self::Input) -> Self::Out {
                cache.eval_live::<LiveStateComponent<FunctionalComponent<$name>>, $out>(input)
            }
        }
    };
    ($name:ident $func_name:ident ($props:ty) -> $out:ty) => {
        function_component!(priv $name $func_name ($props) -> $out);
    }
}

pub trait FCFunction: 'static {
    type Props;
    type Out: Clone + PartialEq;
    fn call<'a>(props: &Self::Props, cache: &FunctionsCache, state: &mut State<'a>) -> Self::Out;
}

pub struct FunctionalComponent<F: FCFunction> {
    props: F::Props,
    manager: RefCell<StateVeriablesManager>,
}

impl<F: FCFunction> Component for FunctionalComponent<F>
{
    type Props = F::Props;
    type Message = ();
    type Out = F::Out;

    fn build(props: Self::Props) -> Self {
        Self {
            props,
            manager: RefCell::new(StateVeriablesManager::new()),
        }
    }

    fn update(&mut self, props: Self::Props) {
        self.props = props;
    }

    fn on_message(&mut self, _message: Self::Message) {
    }

    fn view(&self, link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out {
        let mut state = State {
            current_pos: 0,
            manager: &mut self.manager.borrow_mut(),
            tell_update: Rc::new({
                let link = link.clone();
                move || link.send_message(())
            }),
        };
        F::call(&self.props, cache, &mut state)
    }
}

pub struct StateVeriablesManager {
    state_values: Vec<Rc<dyn Any>>,
}

impl StateVeriablesManager {
    pub fn new() -> Self {
        Self {
            state_values: Vec::new(),
        }
    }
}

pub struct State<'a> {
    current_pos: usize,
    manager: &'a mut StateVeriablesManager,
    tell_update: Rc<dyn Fn()>,
}

impl<'a> State<'a> {
    pub fn use_state<V: 'static>(&mut self, init: impl FnOnce() -> V) -> UseStateHandle<V> {
        // TODO this implementation is not very generic
        // implement the generic hooks and use_reducer, implement use_state in terms of use_reducer and hooks

        let (value, value_ref) = if self.current_pos < self.manager.state_values.len() {
            let value_ref = self.manager.state_values[self.current_pos].clone().downcast::<RefCell<Rc<V>>>().unwrap();
            self.current_pos += 1;
            let value = value_ref.borrow().clone();
            (value, value_ref)
        } else {
            let value = Rc::new(init());
            let value_ref = Rc::new(RefCell::new(value.clone()));
            self.manager.state_values.push(value_ref.clone());
            self.current_pos = self.manager.state_values.len();
            (value, value_ref)
        };

        UseStateHandle {
            value,
            setter: Rc::new({
                let tell_update = self.tell_update.clone();
                move |value| {
                    *value_ref.borrow_mut() = Rc::new(value);
                    tell_update();
                }
            }),
        }
    }
}

impl<'a> Drop for State<'a> {
    fn drop(&mut self) {
        assert_eq!(self.current_pos, self.manager.state_values.len());
    }
}

#[derive(Clone)]
pub struct UseStateHandle<V> {
    value: Rc<V>,
    setter: Rc<dyn Fn(V)>,
}

impl<V: Debug> Debug for UseStateHandle<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<V> UseStateHandle<V> {
    pub fn set(&self, value: V) {
        (self.setter)(value);
    }
}

impl<V> Deref for UseStateHandle<V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

trait Hook {
    type Out;
}