//#![windows_subsystem = "windows"]

use std::{rc::Rc, cell::RefCell};

use regui::{kits::nwg::{components::{Button, self, WindowingStateProps, StateProps, StateChildComponent, WindowSettings, Label, GridLayoutListBuilder, GridLayout, TextInput}, NwgCtx, NwgChildComponent, ChildListBuilder, WrapIntoNwgChildComponent, Application}, Callback, functional_component::{UiBuilder, StateLink}, component::ComponentProps};

extern crate native_windows_gui as nwg;

fn main() {
    let app = Application::new();
    app.ctx().run_ui(Ui {});
}

struct Ui {
}

impl WindowingStateProps for Ui {
    type State = String;
    fn build_state(self, _old_state: Option<Self::State>) -> Self::State {
        "INITIAL".into()
    }
    fn build_ui(builder: &UiBuilder<NwgCtx>, state: &Self::State, link: StateLink<Self::State, NwgCtx>) {
        let set_title = Callback::from(move |title: String| link.update(|s| *s = title));
        builder.get(components::Window {
            content: Example {
                title: state.clone(),
                set_title,
                ..Default::default()
            },
            settings: WindowSettings {
                title: state.clone().into(),
                ..Default::default()
            }
        });
    }
}

fn some_button() -> Button {
    Button {
        text: "some_button".into(),
        position: Some((0, 75)),
        ..Default::default()
    }
}

#[derive(Clone)]
struct Example {
    title: String,
    set_title: Callback<String>,
    close_window: Callback,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            title: "hello".into(),
            set_title: Callback::from(|_| {}),
            close_window: Callback::no_args(|| {}),
        }
    }
}

trait T {
    type Out<'s> where Self: 's;
    fn a<'s>(&'s self) -> Self::Out<'s>;
}

struct A;
impl T for A {
    type Out<'s> = &'static str;
    fn a<'s>(&'s self) -> Self::Out<'s> {
        "hello"
    }
}

impl StateProps for Example {
    type State = (String, Callback, Callback<String>);
    type Out = Rc<RefCell<dyn NwgChildComponent>>;
    fn build_state(self, _old_state: Option<Self::State>) -> Self::State {
        (self.title, self.close_window, self.set_title)
    }
    fn build_ui(builder: &UiBuilder<NwgCtx>, state: &Self::State, link: StateLink<Self::State, NwgCtx>) -> Self::Out {
        let set_title = state.2.clone();
        let set_text = Callback::from(move |text: String| {
            link.update(|s| {
                s.0 = text.clone();
            });
            set_title.call(text.clone());
        });
        let mut list_builder = GridLayoutListBuilder::new(builder)
            .with((0, 0).into(), Button {
                text: "hello".into(),
                on_click: Callback::no_args({
                    let set_text = set_text.clone();
                    move || set_text.call("hello".into())
                }),
                ..Default::default()
            })
            .with((1, 0).into(), Button {
                text: "world".into(),
                position: Some((100, 0)),
                on_click: Callback::no_args({
                    let set_text = set_text.clone();
                    move || set_text.call("world".into())
                }),
                ..Default::default()
            })
            .with((0, 1, 2, 1).into(), Button {
                text: "CLOSE".into(),
                position: Some((0, 25)),
                size: Some((200, 50)),
                on_click: state.1.clone(),
                ..Default::default()
            })
            .with((0, 2, 2, 1).into(), TextInput {
                text: format!("title: {}", state.0).into(),
                position: Some((0, 100)),
                size: Some((400, 25)),
                on_changed: Callback::from(move |text: String| {
                    println!("text changed: {}", text);
                    set_text.call(text.clone());
                }),
                ..Default::default()
            })
            .with((0, 3).into(), Label {
                text: format!("title: {}", state.0).into(),
                position: Some((0, 100)),
                size: Some((400, 25)),
                ..Default::default()
            });
        /*let mut builder = ChildListBuilder::new(builder)
            .with(Button {
                text: "hello".into(),
                on_click: Callback::no_args({
                    let set_text = set_text.clone();
                    move || set_text.call("hello".into())
                }),
                ..Default::default()
            })
            .with(Button {
                text: "world".into(),
                position: Some((100, 0)),
                on_click: Callback::no_args({
                    let set_text = set_text.clone();
                    move || set_text.call("world".into())
                }),
                ..Default::default()
            })
            .with(Button {
                text: "CLOSE".into(),
                position: Some((0, 25)),
                size: Some((200, 50)),
                on_click: state.1.clone(),
                ..Default::default()
            })
            .with(Label {
                text: format!("title: {}", state.0).into(),
                position: Some((0, 100)),
                size: Some((400, 25)),
                ..Default::default()
            });
        */

        if state.0 == "hello" {
            //builder.add(some_button());
            list_builder.add((0, 4).into(), some_button());
        }


        //builder
        //    .build()
        //    .wrap()
        builder.get(GridLayout {
            children: list_builder.build(),
            ..Default::default()
        })
    }
}

impl ComponentProps<NwgCtx> for Example {
    type AssociatedComponent = StateChildComponent<Self>;
}