use std::{rc::Rc, cell::RefCell};

//use nwg::WindowFlags;
//use regui::Callback;

use native_windows_gui as nwg;
use regui::{kits::nwg::{components::Button, NwgControlNode}, component::{Component, LiveValue, LiveLink, ComponentProps}, functional_component::{ComponentsCache, StateLink, StateManager}};

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

    let (_out, _component) = LiveStateComponent::<MyWindow>::build(MyWindow {});

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

struct MyWindow {
}

impl StateProps for MyWindow {
    type State = ();
    type Data = nwg::Window;
    type Out = ();

    fn build_state(self, old_state: Option<Self::State>) -> Self::State {
        ()
    }
    fn build_data(data: Option<Self::Data>, state: &Self::State, link: StateLink<Self::State>, cache: &ComponentsCache) -> (Self::Data, bool) {
        let window = data.unwrap_or_else(|| {
            let mut window = nwg::Window::default();

            nwg::Window::builder()
                //.flags(WindowFlags::MAIN_WINDOW)
                .size((300, 115))
                .title("Hello")
                .build(&mut window)
                .expect("Failed to build window");

            window
        });

        let nodes = cache.get(MyComp);
        let (nodes, emitter) = nodes.into_tuple();
        for node in nodes {
            let _ = node.borrow_mut().handle_from_parent(&window.handle);
        }

        //emitter.listen(move || {
        //    link.update(|_| {});
        //});

        (window, true)
    }
    fn build_result(data: &Self::Data) -> Self::Out {
        ()
    }
}

impl ComponentProps for MyWindow {
    type AssociatedComponent = LiveStateComponent<MyWindow>;
}

#[derive(Clone)]
struct MyComp;

impl ComponentProps for MyComp {
    type AssociatedComponent = LiveStateComponent<MyComp>;
}

/*impl StateProps for MyComp {
    type State = ();
    type Out = components::Label;
    fn build_state(self, old_state: Option<Self::State>) -> Self::State {
        ()
    }
    fn build_ui(builder: &ComponentsCache, state: &Self::State, link: StateLink<Self::State>) -> Self::Out {
        components::Label {
            text: "ciao".into(),
            ..Default::default()
        }
    }
}*/

impl StateProps for MyComp {
    type State = String;
    type Data = Vec<NwgControlNode>;
    type Out = Vec<NwgControlNode>;
    fn build_state(self, old_state: Option<Self::State>) -> Self::State {
        println!("BUILD STATE: {:?}", old_state);
        old_state.unwrap_or("ciao".into())
    }
    fn build_data(mut old_data: Option<Self::Data>, state: &Self::State, link: StateLink<Self::State>, cache: &ComponentsCache) -> (Self::Data, bool) {
        println!("BUILD DATA STATE: {:?}", state);
        let mut v = vec![
            cache.get(Button {
                text: state.clone(),
                on_click: Rc::new({
                    let link = link.clone();
                    move || {
                        link.update(|state| {
                            state.push_str("a");
                        });
                    }
                }),
                ..Default::default()
            }),
            cache.get(Button {
                text: state.clone(),
                position: Some((10, 50)),
                on_click: Rc::new(move || {
                    link.update(|state| {
                        state.push_str("b");
                        nwg::stop_thread_dispatch();
                    });
                }),
                ..Default::default()
            })
        ];
        if state.len() % 2 == 0 {
            v.push(cache.get(Button {
                text: state.clone(),
                position: Some((0, 75)),
                ..Default::default()
            }));
        }

        //println!("DATA: {:?}", link.live_value().current_value().len());
        //println!("DATA: {:?}", link.live_value().current_value().len());

        (v, true)
    }
    fn build_result(data: &Self::Data) -> Self::Out {
        println!("BUILD RESULT");
        data.clone()
    }
}

pub trait StateProps: 'static { // TODO remove 'static
    type State;
    type Data;
    type Out;
    #[must_use]
    fn build_state(self, old_state: Option<Self::State>) -> Self::State;
    #[must_use]
    fn build_data(data: Option<Self::Data>, state: &Self::State, link: StateLink<Self::State>, cache: &ComponentsCache) -> (Self::Data, bool);
    #[must_use]
    fn build_result(data: &Self::Data) -> Self::Out;
}

struct LiveStateComponent<Props: StateProps> {
    state_manager: StateManager<Props::State>,
    components_cache: Rc<RefCell<ComponentsCache>>,
    data: Rc<RefCell<Option<Props::Data>>>,
    live_link: LiveLink,
}

impl<Props: StateProps> Component for LiveStateComponent<Props> {
    type Props = Props;
    type Output = LiveValue<Props::Out>;
    fn build(props: Self::Props) -> (Self::Output, Self) {

        let state = props.build_state(None);
        let state_manager = StateManager::<Props::State>::new_with(state);
        let components_cache = Rc::new(RefCell::new(ComponentsCache::new()));

        let state = state_manager.take_state();
        let (data, _rerender) = Props::build_data(None, state.as_ref().unwrap(), state_manager.link(), &components_cache.borrow_mut());
        components_cache.borrow_mut().finish();
        state_manager.link().set(state.unwrap());
        let result = Props::build_result(&data);
        let data = Rc::new(RefCell::new(Some(data)));

        let live_link = LiveLink::new();

        state_manager.set_builder({
            let components_cache = components_cache.clone();
            let to_rebuild = Rc::new(RefCell::new(false));
            let data = data.clone();
            let live_link = live_link.clone();
            move |state, link| {
                if let Ok(mut components_cache) = components_cache.try_borrow_mut() {
                    *to_rebuild.borrow_mut() = true;
                    let mut to_rerender = false;
                    while to_rebuild.borrow().clone() {
                        *to_rebuild.borrow_mut() = false;
                        {
                            let mut data = data.borrow_mut();
                            let data_content = data.take();
                            assert!(data_content.is_some());
                            let (data_content, rerender) = Props::build_data(data_content, state, link.clone(), &components_cache);
                            to_rerender |= rerender;
                            components_cache.finish();
                            data.replace(data_content);
                        }
                        // TODO get_live
                        if to_rerender {
                            live_link.tell_update();
                        }
                    }
                } else {
                    *to_rebuild.borrow_mut() = true;
                }
            }
        });

        (
            live_link.make_live_value(result),
            Self {
                state_manager,
                components_cache,
                data,
                live_link,
            }
        )
    }
    fn changed(&mut self, props: Self::Props) -> Self::Output {
        //let state = props.build_state(self.state_manager.take_state());
        //self.component.borrow_mut().changed(Props::build_ui(&ComponentsCache::new(), &state, self.state_manager.link()));
        let data = self.data.borrow();
        let result = Props::build_result(data.as_ref().unwrap());
        self.live_link.make_live_value(result)
    }
    //fn eval(&mut self, input: Self::Input) -> Self::Output {
    //    self.component.borrow_mut().eval(input)
    //}
}