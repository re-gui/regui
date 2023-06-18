use std::{rc::Rc, cell::RefCell};

//use nwg::WindowFlags;
//use regui::Callback;

use native_windows_gui as nwg;
use regui::{kits::nwg::{components::Button, NwgControlNode}, state_function::{StateFunction, LiveValue, LiveLink, StateFunctionProps}, functional_component::{StateLink, StateManager, FunctionsCache}};

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let (_out, _component) = LiveStateComponent::<MyWindowState>::build(MyWindowProps {});

    nwg::dispatch_thread_events();
}

struct MyWindowProps {
}

struct MyWindowState {
    window: nwg::Window,
}

impl Component for MyWindowState {
    type Props = MyWindowProps;
    type Out = ();

    fn build(_props: Self::Props) -> Self {
        let mut window = nwg::Window::default();

        nwg::Window::builder()
            //.flags(WindowFlags::MAIN_WINDOW)
            .size((300, 115))
            .title("Hello")
            .build(&mut window)
            .expect("Failed to build window");

        Self {
            window,
        }
    }
    fn update(&mut self, _props: Self::Props) {
    }
    fn view(&self, _link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out {
        let nodes = cache.eval_live(MyCompProps);
        for node in nodes {
            let _ = node.borrow_mut().handle_from_parent(&self.window.handle);
        }
        ()
    }
}

impl StateFunctionProps for MyWindowProps {
    type AssociatedComponent = LiveStateComponent<MyWindowState>;
}

#[derive(Clone)]
struct MyCompProps;

#[derive(Clone)]
struct MyCompState {
    text: String,
}

impl StateFunctionProps for MyCompProps {
    type AssociatedComponent = LiveStateComponent<MyCompState>;
}

impl Component for MyCompState {
    type Props = MyCompProps;
    type Out = Vec<NwgControlNode>;
    fn build(_props: Self::Props) -> Self {
        Self {
            text: "ciao".into(),
        }
    }
    fn update(&mut self, _props: Self::Props) {
    }
    fn view(&self, link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out {
        println!("view");
        let mut v = vec![
            cache.eval(Button {
                text: self.text.clone(),
                on_click: Rc::new({
                    let link = link.clone();
                    move || {
                        link.update(|state| {
                            state.text.push_str("a");
                        });
                    }
                }),
                ..Default::default()
            }),
            cache.eval(Button {
                text: self.text.clone(),
                position: Some((10 * self.text.len() as i32 - 40, 50)),
                on_click: Rc::new(move || {
                    link.update(|state| {
                        state.text.push_str("b");
                        nwg::stop_thread_dispatch();
                    });
                }),
                ..Default::default()
            })
        ];
        if self.text.len() % 2 == 0 {
            v.push(cache.eval(Button {
                text: self.text.clone(),
                position: Some((0, 75)),
                ..Default::default()
            }));
        }

        v
    }
}

pub trait Component: Sized + 'static { // TODO remove 'static
    type Props;
    type Out: PartialEq + Clone + 'static;
    #[must_use]
    fn build(props: Self::Props) -> Self;
    fn update(&mut self, props: Self::Props);
    #[must_use]
    fn view(&self, link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out;
}

struct LiveStateComponent<SC: Component> {
    state_manager: Rc<RefCell<StateManager<SC>>>,
    components_cache: Rc<RefCell<FunctionsCache>>,
    out: Rc<RefCell<SC::Out>>,
    live_link: LiveLink,
}

impl<SC: Component> StateFunction for LiveStateComponent<SC> {
    type Props = SC::Props;
    type Output = LiveValue<SC::Out>;
    fn build(props: Self::Props) -> (Self::Output, Self) {

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
                link.update(|_| {});
            }
        });

        let state_manager = Rc::new(RefCell::new(state_manager));

        state_manager.borrow_mut().set_builder({
            let cache = components_cache.clone();
            let out = out.clone();
            let live_link = live_link.clone();
            let state_manager = state_manager.clone();
            move |link| {
                let mut cache = cache.borrow_mut();
                let new_result = {
                    let result = state_manager.borrow().on_state(|component| {
                        component.view(link.clone(), &cache)
                    });
                    cache.finish();
                    result
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
    fn changed(&mut self, props: Self::Props) -> Self::Output {
        self.state_manager.borrow().on_mut_state(|component| {
            component.update(props);
        });

        // TODO when a component internally updates its state, it will use this update function.
        // this will cause the parent's view function to be called, which will call the child's changed function (this function).
        // But this function will call update again here, wich will cause the view function to be called again.
        // This will cause the view function to be called twice, which is not good.
        // Maybe just removing this call will be enough, since the the component's update will already call the link update function if needed,
        // but this should be analyzed more in depth.
        self.state_manager.borrow().link().update(|_| {});

        self.live_link.make_live_value(self.out.borrow().clone())
    }
}