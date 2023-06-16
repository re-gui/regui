use std::{any::Any, marker::PhantomData, rc::{Rc, Weak}, cell::{RefCell, Ref}};

use crate::{component::{Component, ComponentProps}, context::Context};



struct FunctionalComponentBase<Props, Ctx> {
    elements: Vec<Rc<dyn Any>>,
    element_pos: usize, // TODO merge
    states: Vec<Rc<dyn Any>>,
    states_pos: usize, // TODO merge
    _phantom: PhantomData<(Props, Ctx)>,
}

#[derive(Debug, Clone)]
struct StateValueHook<T>(Rc<RefCell<T>>);

impl<T> StateValueHook<T> {
    fn borrow_value(&self) -> Ref<T> {
        self.0.borrow()
    }

    fn set_value(&self, value: T) {
        *self.0.borrow_mut() = value;
    }
}

//fn ciao(a: Weak<i32>) {
//    a. 
//}

impl<Props, Ctx> FunctionalComponentBase<Props, Ctx> {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            element_pos: 0,
            states: Vec::new(),
            states_pos: 0,
            _phantom: PhantomData,
        }
    }

    fn reset_state_counters(&mut self) {
        self.states.truncate(self.states_pos);
        self.states_pos = 0;
        self.elements.truncate(self.element_pos);
        self.element_pos = 0;
    }

    fn use_state<T: 'static + Any>(&mut self, init_value: impl FnOnce() -> T) -> (T, fn(T)) {
        let pos = self.states_pos;
        let value = if pos < self.states.len() {
            let value = self.states[pos].clone().downcast::<RefCell<T>>();
            let value = if let Ok(value) = value {
                value.clone()
            } else {
                let value = Rc::new(RefCell::new(init_value()));
                self.states.insert(pos, value.clone());
                value
            };
            self.states_pos += 1;
            value
        } else {
            let value = Rc::new(RefCell::new(init_value()));
            self.states.push(value.clone());
            self.states_pos = self.states.len();
            value
        };
        
        let setter_value = value.clone();

        todo!()
    }
}

impl<Props: ComponentProps, Ctx: Context> Component<Ctx> for FunctionalComponentBase<Props, Ctx> {
    type Props = Props;

    fn build(ctx: &Ctx, props: Self::Props) -> Self {
        todo!()
    }

    fn changed(&mut self, props: Self::Props, ctx: &Ctx) {
        todo!()
    }
}

