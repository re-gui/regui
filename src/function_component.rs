use std::{rc::Rc, fmt::Debug, any::Any, cell::RefCell};

use crate::component::{FunctionsCache, Component, StateLink, LiveStateComponent};

pub struct Cx<'a, 'b> {
    cache: &'a FunctionsCache,
    state: &'a mut State<'b>,
}

impl<'a, 'b> Cx<'a, 'b> {
    pub fn new(cache: &'a FunctionsCache, state: &'a mut State<'b>) -> Self {
        Self {
            cache,
            state,
        }
    }

    pub fn cache(&self) -> &'a FunctionsCache {
        self.cache
    }

    pub fn use_state<V: 'static>(&mut self, init: impl FnOnce() -> V) -> UseStateHandle<V> {
        self.state.use_state(init)
    }

    pub fn use_ref<V: 'static>(&mut self, init: impl FnOnce() -> V) -> Rc<V> {
        self.state.use_ref(init)
    }
}

/// Declares a function component
///
/// # Example
/// ```ignore
/// function_component!(pub MyComponent my_component(i32) -> Vec<NwgControlNode>);
/// fn my_component(props: &i32, cx: &mut Cx) -> Vec<NwgControlNode> {
///     // ...
/// }
/// ```
///
/// Note that the following are valid:
/// - `function_component!(pub MyComponent ...)` declares a public component
/// - `function_component!(priv MyComponent ...)` declares a private component
/// - `function_component!(MyComponent ...)` also declares a private component
/// 
/// You can also embed doc comments:
/// ```ignore
/// function_component!(
///     /// This is a component
///     pub MyComponent my_component(i32) -> Vec<NwgControlNode>
/// );
/// ```
#[macro_export]
macro_rules! decl_function_component {
    ($(#[$outer:meta])* pub $name:ident $func_name:ident ($props:ty) -> $out:ty) => {
        $(#[$outer])*
        pub struct $name;
        impl $crate::function_component::ComponentFunction for $name {
            type Props = $props;
            type Out = $out;
            fn call<'a>(props: &Self::Props, cache: &$crate::component::FunctionsCache, state: &mut $crate::function_component::State<'a>) -> Self::Out {
                let mut cx = $crate::function_component::Cx::new(cache, state);
                $func_name(props, &mut cx)
            }
        }
    };
    ($(#[$outer:meta])* priv $name:ident $func_name:ident ($props:ty) -> $out:ty) => {
        $(#[$outer])*
        struct $name;
        impl $crate::function_component::ComponentFunction for $name {
            type Props = $props;
            type Out = $out;
            fn call<'a>(props: &Self::Props, cache: &$crate::component::FunctionsCache, state: &mut State<'a>) -> Self::Out {
                //$func_name(props, cache, state)
                let mut cx = $crate::function_component::Cx::new(cache, state);
                $func_name(props, &mut cx)
            }
        }
    };
    ($(#[$outer:meta])* $name:ident $func_name:ident ($props:ty) -> $out:ty) => {
        decl_function_component!($(#[$outer])* priv $name $func_name ($props) -> $out);
    }
}

pub trait ComponentFunction: 'static + Sized {
    type Props;
    type Out: Clone + PartialEq;
    fn call<'a>(props: &Self::Props, cache: &FunctionsCache, state: &mut State<'a>) -> Self::Out;
    fn eval(cx: &mut Cx, props: Self::Props) -> Self::Out {
        cx.cache().eval_live::<LiveStateComponent<FunctionComponent<Self>>, Self::Out>(props)
    }
}

pub struct FunctionComponent<F: ComponentFunction> {
    props: F::Props,
    manager: RefCell<StateVeriablesManager>,
}

impl<F: ComponentFunction> Component for FunctionComponent<F>
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

        let value = if self.current_pos < self.manager.state_values.len() {
            let value = self.manager.state_values[self.current_pos].clone().downcast::<RefCell<V>>().unwrap();
            self.current_pos += 1;
            value
        } else {
            let value = Rc::new(RefCell::new(init()));
            self.manager.state_values.push(value.clone());
            self.current_pos = self.manager.state_values.len();
            value
        };

        UseStateHandle {
            value: value.clone(),
            setter: Rc::new({
                let tell_update = self.tell_update.clone();
                let value_ref = value.clone();
                move |value| {
                    *value_ref.borrow_mut() = value;
                    tell_update();
                }
            }),
        }
    }

    pub fn use_ref<V: 'static>(&mut self, init: impl FnOnce() -> V) -> Rc<V> {
        // TODO this implementation is not very generic
        // implement the generic hooks and use_reducer, implement use_state in terms of use_reducer and hooks

        let value = if self.current_pos < self.manager.state_values.len() {
            let value = self.manager.state_values[self.current_pos].clone().downcast::<V>().unwrap();
            self.current_pos += 1;
            value
        } else {
            let value = Rc::new(init());
            self.manager.state_values.push(value.clone());
            self.current_pos = self.manager.state_values.len();
            value
        };

        value
    }
}

impl<'a> Drop for State<'a> {
    fn drop(&mut self) {
        assert_eq!(self.current_pos, self.manager.state_values.len());
    }
}

#[derive(Clone)]
pub struct UseStateHandle<V> {
    value: Rc<RefCell<V>>,
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
    pub fn get(&self) -> V
    where
        V: Clone,
    {
        self.value.borrow().clone()
    }
    pub fn on_value<Out>(&self, callback: impl FnOnce(&V) -> Out) -> Out {
        let value = self.value.borrow();
        callback(&*value)
    }
    pub fn on_mut_value<Out>(&self, callback: impl FnOnce(&mut V) -> Out) -> Out {
        let mut value = self.value.borrow_mut();
        callback(&mut *value)
    }
}

trait Hook {
    type Out;
}