use std::{error::Error, rc::Rc, cell::RefCell, ops::Deref,};

//use nwg::WindowFlags;
//use regui::Callback;

use native_windows_gui as nwg;
use regui::{kits::nwg::{NativeCommonComponentComponent, NativeCommonComponent, components::{self, LabelComponent, Label, Button}, NwgControlNode}, component::{Component, ComponentProps, Live, LiveValue, LiveLink}, functional_component::{ComponentsCache, StateLink, StateManager}};

fn main() {
    //let cb = Callback::from(|x: i32, y: ()| x + 1);
    //let c = Callback::from(|| {});

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    //let (node, _component) = Label {
    //    text: "ciao".into(),
    //    ..Default::default()
    //}.build();

    let mut window = nwg::Window::default();

    nwg::Window::builder()
        //.flags(WindowFlags::MAIN_WINDOW)
        .size((300, 115))
        .title("Hello")
        .build(&mut window)
        .expect("Failed to build window");

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

    exec(window.handle, MyComp);

    //nwg::dispatch_thread_events();

    //println!("{:?}", c);
    //println!("{:?}", cb);
}

fn exec<Props>(window_handle: nwg::ControlHandle, props: Props)
where
    Props: StateProps,
    Props::Out: Live<Vec<NwgControlNode>>,
{
    let (node, _component) = StateComponent::build(props);

    let node = Rc::new(node);
    let refs = Rc::new(RefCell::new(vec![]));
    for node in node.deref().current_value() {
        let r = node.0.borrow_mut().handle_from_parent(&window_handle);
        println!("{:?}", r.handle());
        refs.borrow_mut().push(r);
    }
    //node.listen({
    //    let node = node.clone();
    //    let refs = refs.clone();
    //    let window_handle = window_handle.clone();
    //    move || {
    //        refs.borrow_mut().clear();
    //        for node in node.deref().current_value() {
    //            let r = node.0.borrow_mut().handle_from_parent(&window_handle);
    //            println!("{:?}", r.handle());
    //            //refs.borrow_mut().push(r);
    //        }
    //    }
    //});

    //let r = node.current_value().0.borrow_mut().handle_from_parent(&window_handle);
    //println!("{:?}", r.handle());
    nwg::dispatch_thread_events();
    //component.mount(window_handle);
}

struct MyComp;

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
    type Data = LiveLink<Vec<NwgControlNode>>;
    type Out = LiveValue<Vec<NwgControlNode>>;
    fn build_state(self, old_state: Option<Self::State>) -> Self::State {
        println!("BUILD STATE");
        "ciao".into()
    }
    fn build_data(mut data: Option<Self::Data>, state: &Self::State, link: StateLink<Self::State>, cache: &ComponentsCache) -> Self::Data {
        println!("BUILD DATA");
        let v = vec![
            cache.get(Button {
                text: state.clone(),
                on_click: Rc::new({
                    let link = link.clone();
                    move || {
                        link.update(|state| {
                            println!("ciao");
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
                        println!("ciao B");
                        state.push_str("b");
                        nwg::stop_thread_dispatch();
                    });
                }),
                ..Default::default()
            })
        ];
        let link = match data {
            Some(data) => {
                data.set(move || v.clone());
                data
            },
            None => LiveLink::new(move || v.clone()),
        };

        println!("DATA: {:?}", link.live_value().current_value().len());
        println!("DATA: {:?}", link.live_value().current_value().len());

        link
    }
    fn build_result(data: &Self::Data) -> Self::Out {
        println!("BUILD RESULT");
        data.live_value()
    }
}

pub trait StateProps: 'static { // TODO remove 'static
    type State;
    type Data;
    type Out;
    #[must_use]
    fn build_state(self, old_state: Option<Self::State>) -> Self::State;
    #[must_use]
    fn build_data(data: Option<Self::Data>, state: &Self::State, link: StateLink<Self::State>, cache: &ComponentsCache) -> Self::Data;
    #[must_use]
    fn build_result(data: &Self::Data) -> Self::Out;
}

struct StateComponent<Props: StateProps> {
    state_manager: StateManager<Props::State>,
    components_cache: Rc<RefCell<ComponentsCache>>,
    data: Rc<RefCell<Option<Props::Data>>>,
}

impl<Props: StateProps> Component for StateComponent<Props> {
    type Props = Props;
    type Output = Props::Out;
    fn build(props: Self::Props) -> (Self::Output, Self) {

        let state = props.build_state(None);
        let state_manager = StateManager::<Props::State>::new_with(state);
        let components_cache = Rc::new(RefCell::new(ComponentsCache::new()));

        let state = state_manager.take_state();
        let data = Props::build_data(None, state.as_ref().unwrap(), state_manager.link(), &components_cache.borrow_mut());
        components_cache.borrow_mut().finish();
        state_manager.link().set(state.unwrap());
        let result = Props::build_result(&data);
        let data = Rc::new(RefCell::new(Some(data)));

        state_manager.set_builder({
            let components_cache = components_cache.clone();
            let to_rebuild = Rc::new(RefCell::new(false));
            let data = data.clone();
            move |state, link| {
                if let Ok(mut components_cache) = components_cache.try_borrow_mut() {
                    *to_rebuild.borrow_mut() = true;
                    while to_rebuild.borrow().clone() {
                        *to_rebuild.borrow_mut() = false;
                        let data_content = data.borrow_mut().take();
                        assert!(data_content.is_some());
                        let data_content = Props::build_data(data_content, state, link.clone(), &components_cache);
                        components_cache.finish();
                        data.borrow_mut().replace(data_content);
                    }
                } else {
                    *to_rebuild.borrow_mut() = true;
                }
            }
        });

        (
            result,
            Self {
                state_manager,
                components_cache,
                data,
            }
        )
    }
    fn changed(&mut self, props: Self::Props) -> Self::Output {
        let state = props.build_state(self.state_manager.take_state());
        //self.component.borrow_mut().changed(Props::build_ui(&ComponentsCache::new(), &state, self.state_manager.link()));
        let data = self.data.borrow();
        let result = Props::build_result(data.as_ref().unwrap());
        result
    }
    //fn eval(&mut self, input: Self::Input) -> Self::Output {
    //    self.component.borrow_mut().eval(input)
    //}
}