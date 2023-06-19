use std::rc::Rc;


use native_windows_gui as nwg;
use regui::{component::{LiveStateComponent, Component, StateLink, FunctionsCache}, StateFunctionProps};
use regui_nwg::{NwgControlNode, components::{Button, Label, TextInput, Window}};

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    //let (_out, _component) = MyWindowProps {}.build();
    let (_out, _component) = MyUi.build();

    nwg::dispatch_thread_events();
}

struct MyUi;

impl StateFunctionProps for MyUi {
    type AssociatedFunction = LiveStateComponent<MyUiState>;
}

struct MyUiState {
    title: String,
    icon: Rc<nwg::Icon>,
}

impl Component for MyUiState {
    type Props = MyUi;
    type Out = ();
    type Message = String;

    fn build(_props: Self::Props) -> Self {
        const LOGO_PNG: &[u8] = include_bytes!("../../../logo.png");
        let icon = nwg::Icon::from_bin(LOGO_PNG).expect("Failed to load icon");
        Self {
            title: "title".into(),
            icon: Rc::new(icon)
        }
    }

    fn on_message(&mut self, message: Self::Message) {
        self.title = message;
    }

    fn view(&self, link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out {
        let set_title = {
            let link = link.clone();
            move |text: &str| link.send_message(text.into())
        };

        let _ = cache.eval(Window {
            title: self.title.clone(),
            content: cache.live(MyCompProps {
                change_text: Rc::new(set_title),
            }).into(),
            on_close_request: Rc::new(|| nwg::stop_thread_dispatch()),
            icon: if self.title.len() % 2 == 0 { Some(self.icon.clone()) } else { None },
            ..Default::default()
        });
    }
}

struct MyCompProps {
    change_text: Rc<dyn Fn(&str)>,
}

#[derive(Clone)]
struct MyCompState {
    text: String,
    change_text: Rc<dyn Fn(&str)>,
}

impl StateFunctionProps for MyCompProps {
    type AssociatedFunction = LiveStateComponent<MyCompState>;
}

enum MyMessage {
    SetTitle(String),
}

impl Component for MyCompState {
    type Props = MyCompProps;
    type Out = Vec<NwgControlNode>;
    type Message = MyMessage;
    fn build(props: Self::Props) -> Self {
        let title = "Hello world!";
        (props.change_text)(title);
        Self {
            text: title.into(),
            change_text: props.change_text,
        }
    }
    fn update(&mut self, props: Self::Props) {
        self.change_text = props.change_text;
    }
    fn on_message(&mut self, message: Self::Message) {
        match message {
            MyMessage::SetTitle(text) => {
                self.text = text;
                (self.change_text)(&self.text);
            }
        }
    }
    fn view(&self, link: StateLink<Self>, cache: &FunctionsCache) -> Self::Out {
        //println!("view");
        let mut v = vec![
            cache.eval(Label {
                text: "window title:".into(),
                position: Some((0, 0)),
                size: Some((100, 25)),
                ..Default::default()
            }),
            cache.eval(TextInput {
                text: self.text.clone(),
                position: Some((100, 0)),
                size: Some((100, 25)),
                on_user_input: Rc::new({
                    let link = link.clone();
                    move |text| link.send_message(MyMessage::SetTitle(text.into()))
                }),
                ..Default::default()
            }),
            cache.eval(Button {
                text: "CLOSE".into(),
                position: Some((0, 25)),
                on_click: Rc::new(|| nwg::stop_thread_dispatch()),
                ..Default::default()
            }),
        ];
        if self.text.len() % 2 == 0 {
            v.push(cache.eval(Button {
                text: format!("{} % 2 = 0", self.text.len()),
                position: Some((0, 75)),
                ..Default::default()
            }));
        }
        v.push(cache.eval(Button {
            text: format!("{} % 2 = 1", self.text.len()),
            position: Some((100, 75)),
            enabled: self.text.len() % 2 == 1,
            ..Default::default()
        }));

        v
    }
}
