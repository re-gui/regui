use std::rc::Rc;


use native_windows_gui as nwg;
use regui::{component::{LiveStateComponent, Component, StateLink, FunctionsCache}, StateFunctionProps, StateFunction};
use regui_nwg::{NwgControlNode, components::{Button, Label, TextInput}};

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let (_out, _component) = MyWindowProps {}.build();

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
    type Message = ();

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
    fn update(&mut self, _props: Self::Props) {}
    fn on_message(&mut self, _message: Self::Message) {}
    fn view(&self, _link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out {
        let nodes = cache.live(MyCompProps);
        for node in nodes {
            let _ = node.borrow_mut().handle_from_parent(&self.window.handle);
        }
        ()
    }
}

impl StateFunctionProps for MyWindowProps {
    type AssociatedFunction = LiveStateComponent<MyWindowState>;
}

#[derive(Clone)]
struct MyCompProps;

#[derive(Clone)]
struct MyCompState {
    text: String,
}

impl StateFunctionProps for MyCompProps {
    type AssociatedFunction = LiveStateComponent<MyCompState>;
}

enum MyMessage {
    SetText(String),
}

impl Component for MyCompState {
    type Props = MyCompProps;
    type Out = Vec<NwgControlNode>;
    type Message = MyMessage;
    fn build(_props: Self::Props) -> Self {
        Self {
            text: "ciao".into(),
        }
    }
    fn update(&mut self, _props: Self::Props) {}
    fn on_message(&mut self, message: Self::Message) {
        match message {
            MyMessage::SetText(text) => {
                self.text = text;
            }
        }
    }
    fn view(&self, link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out {
        //println!("view");
        let mut v = vec![
            cache.eval(Button {
                text: "PUSH".into(),
                on_click: Rc::new({
                    let link = link.clone();
                    move || link.send_update(|state| state.text.push_str("a"))
                }),
                ..Default::default()
            }),
            cache.eval(Button {
                text: "POP".into(),
                position: Some((100, 0)),
                on_click: Rc::new({
                    let link = link.clone();
                    move || link.send_update(|state| { state.text.pop(); })
                }),
                ..Default::default()
            }),
            cache.eval(TextInput {
                text: self.text.clone(),
                position: Some((200, 0)),
                on_user_input: Rc::new({
                    let link = link.clone();
                    move |text| {
                        println!("input");
                        link.send_message(MyMessage::SetText(text.into()));
                    }
                }),
                ..Default::default()
            }),
            cache.eval(Label {
                text: self.text.clone(),
                position: Some((0, 25)),
                ..Default::default()
            }),
            cache.eval(Button {
                text: "CLOSE".into(),
                position: Some((10 * self.text.len() as i32 - 40, 50)),
                on_click: Rc::new(move || {
                    link.send_update(|_| {
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
