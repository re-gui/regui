use std::{rc::Rc, cell::RefCell};

//use nwg::WindowFlags;
//use regui::Callback;

use native_windows_gui as nwg;
use regui::{kits::nwg::{components::Button, NwgControlNode}, state_function::{StateFunction, LiveValue, LiveLink, StateFunctionProps}, functional_component::{StateLink, StateManager, FunctionsCache}};

fn main() {
    //let cb = Callback::from(|x: i32, y: ()| x + 1);
    //let c = Callback::from(|| {});

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    //let (node, _component) = Label {
    //    text: "ciao".into(),
    //    ..Default::default()
    //}.build();

    //let mut window = nwg::Window::default();

    //nwg::Window::builder()
    //    //.flags(WindowFlags::MAIN_WINDOW)
    //    .size((300, 115))
    //    .title("Hello")
    //    .build(&mut window)
    //    .expect("Failed to build window");

    //let r = node.0.borrow_mut().handle_from_parent(&window.handle);
    //println!("{:?}", r.handle());

    //let (node, _component) = Button {
    //    text: "ciao".into(),
    //    position: Some((10, 50)),
    //    on_click: Rc::new(|| println!("ciao")),
    //    ..Default::default()
    //}.build();

    //let r = node.0.borrow_mut().handle_from_parent(&window.handle);
    //println!("{:?}", r.handle());

    //let (node, _component) = StateComponent::build(MyComp);
    //let r = node.0.borrow_mut().handle_from_parent(&window.handle);

    //exec(window.handle, MyComp);

    let (_out, _component) = LiveStateComponent::<MyWindowState>::build(MyWindowProps {});

    nwg::dispatch_thread_events();

    //println!("{:?}", c);
    //println!("{:?}", cb);
}

//fn exec<Props: Clone>(window_handle: nwg::ControlHandle, props: Props)
//where
//    Props: StateProps<Out = LiveValue<Vec<NwgControlNode>>>,
//{
//    let (nodes, component) = StateComponent::build(props.clone());
//
//    let (nodes, emitter) = nodes.into_tuple();
//
//    for node in nodes {
//        let _ = node.borrow_mut().handle_from_parent(&window_handle);
//    }
//
//    emitter.listen({
//        let window_handle = window_handle.clone();
//        let mut component = component;
//        move || {
//            let nodes = component.changed(props.clone());
//            let (nodes, _emitter) = nodes.into_tuple();
//            for node in nodes {
//                let _ = node.borrow_mut().handle_from_parent(&window_handle);
//            }
//        }
//    });
//
//    //let r = node.current_value().0.borrow_mut().handle_from_parent(&window_handle);
//    //println!("{:?}", r.handle());
//    nwg::dispatch_thread_events();
//    //component.mount(window_handle);
//}

struct MyWindowProps {
}

struct MyWindowState {
    window: nwg::Window,
}

impl Component for MyWindowState {
    type Props = MyWindowProps;
    type Out = ();

    fn build(props: Self::Props) -> Self {
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
    fn update(&mut self, props: Self::Props) {
    }
    fn view(&self, link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out {
        let nodes = cache.eval_live(MyCompProps);
        for node in nodes {
            let _ = node.borrow_mut().handle_from_parent(&self.window.handle);
        }

        //emitter.listen(move || {
        //    link.update(|_| {});
        //});

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
    fn build(props: Self::Props) -> Self {
        Self {
            text: "ciao".into(),
        }
    }
    fn update(&mut self, props: Self::Props) {
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

//struct Context<C: Component> {
//}

//pub trait Component: Sized + 'static { // TODO possibly remove 'static
//    type Props: StateProps;
//    fn build(props: Self::Props, ctx: &Context<Self>) -> Self;
//    fn update(&mut self, props: Self::Props, ctx: &Context<Self>);
//}

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
        let cache = Rc::new(RefCell::new(FunctionsCache::new()));

        let result = {
            let result = state_manager.on_state(|component| {
                component.view(state_manager.link(), &cache.borrow_mut())
            });
            cache.borrow_mut().finish();
            result
        };
        let out = Rc::new(RefCell::new(result.clone()));

        let live_link = LiveLink::new();

        cache.borrow_mut().emitter().listen({
            let link = state_manager.link();
            move || {
                link.update(|_| {});
            }
        });

        let state_manager = Rc::new(RefCell::new(state_manager));

        state_manager.borrow_mut().set_builder({
            let cache = cache.clone();
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
                components_cache: cache,
                out,
                live_link,
            }
        )
    }
    fn changed(&mut self, props: Self::Props) -> Self::Output {
        self.state_manager.borrow().on_mut_state(|component| {
            component.update(props);
        });
        // TODO run
        self.live_link.make_live_value(self.out.borrow().clone())
    }
    //fn eval(&mut self, input: Self::Input) -> Self::Output {
    //    self.component.borrow_mut().eval(input)
    //}
}