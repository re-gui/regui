#![windows_subsystem = "windows"]

use std::rc::Rc;


use native_windows_gui as nwg;
use regui::component::{LiveStateComponent, Component, StateLink, FunctionsCache, GetFromCache};
use regui_nwg::{NwgNode, components::{Window, Button, Label, TextInput}, run_ui, WindowEvent};

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    run_ui::<UiState>(Ui);
}

struct Ui;

struct UiState {
    title: String,
    icon: Rc<nwg::Icon>,
}

impl Component for UiState {
    type Props = Ui;
    type Out = ();
    type Message = String;

    fn build(_props: Self::Props) -> Self {
        const LOGO_PNG: &[u8] = include_bytes!("logo.png");
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

        let _ = Window::builder()
            .title(&self.title)
            .content(WindowContent {
                change_text: Box::new(set_title),
            }.get(cache).into())
            .on_window_event(|event| {
                match event {
                    WindowEvent::CloseRequest => nwg::stop_thread_dispatch(),
                    _ => {}
                }
            })
            .icon_opt(if self.title.len() % 2 == 0 { Some(self.icon.clone()) } else { None })
            .build().get(cache);
    }
}

struct WindowContent {
    change_text: Box<dyn Fn(&str)>,
}

impl GetFromCache for WindowContent {
    type Out = Vec<NwgNode<nwg::ControlHandle>>;
    fn get(self, cache: &FunctionsCache) -> Self::Out {
        cache.eval_live::<LiveStateComponent<WindowContentState>, Self::Out>(self)
    }
}

struct WindowContentState {
    text: String,
    change_text: Box<dyn Fn(&str)>,
}

enum MyMessage {
    SetTitle(String),
}

impl Component for WindowContentState {
    type Props = WindowContent;
    type Out = Vec<NwgNode<nwg::ControlHandle>>;
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
            Label::builder()
                .text("window title:")
                .position(0, 0)
                .size(100, 25)
                .get(cache),
            TextInput::builder()
                .text(&self.text)
                .position(100, 0)
                .size(150, 25)
                .on_user_input({
                    let link = link.clone();
                    move |text| link.send_message(MyMessage::SetTitle(text.into()))
                })
                .get(cache),
            Button::builder()
                .text("CLOSE")
                .position(0, 25)
                .on_click(|| nwg::stop_thread_dispatch())
                .get(cache),
        ];
        if self.text.len() % 2 == 0 {
            v.push(Button::builder()
                .text(format!("{} % 2 = 0", self.text.len()))
                .position(0, 75)
                .get(cache)
            );
        }
        v.push(Button::builder()
            .text(format!("{} % 2 = 1", self.text.len()))
            .position(100, 75)
            .enabled(self.text.len() % 2 == 1)
            .get(cache)
        );

        v
    }
}


