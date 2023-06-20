#![windows_subsystem = "windows"]


use std::{rc::Rc, ops::Deref};

use native_windows_gui as nwg;
use regui::{component::{LiveStateComponent, FunctionsCache, GetFromCache, EvalFromCache}, function_component::{State, FunctionComponent}, function_component};
use regui_nwg::{NwgControlNode, components::{Window, Button, Label, TextInput}, run_ui, WindowEvent};

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    run_ui::<FunctionComponent<ExampleUi>>(());
}

impl GetFromCache for ExampleUi {
    type Out = ();
    fn get(self, cache: &FunctionsCache) -> Self::Out {
        cache.eval_live::<LiveStateComponent<FunctionComponent<ExampleUi>>, ()>(())
    }
}

function_component!(ExampleUi example_ui(()) -> ());

fn example_ui(_props: &(), cache: &FunctionsCache, state: &mut State) -> () {
    let icon = state.use_state(|| {
        const LOGO_PNG: &[u8] = include_bytes!("logo.png");
        let icon = nwg::Icon::from_bin(LOGO_PNG).expect("Failed to load icon");
        Rc::new(icon)
    });

    let (title, content) = ExampleContent::eval(cache, 42);

    let _ = Window::builder()
        .title(&title)
        .content(content.into())
        .on_window_event(|event| {
            match event {
                WindowEvent::CloseRequest => nwg::stop_thread_dispatch(),
                _ => {}
            }
        })
        .icon_opt(if title.len() % 2 == 0 { Some(icon.deref().clone()) } else { None })
        .initial_size(300, 150)
        .build().get(cache);
}

function_component!(ExampleContent example_content(i32) -> (String, Vec<NwgControlNode>));

fn example_content(props: &i32, cache: &FunctionsCache, state: &mut State) -> (String, Vec<NwgControlNode>) {
    let title = state.use_state(|| props.to_string());

    let mut v = vec![
        Label::builder()
            .text("window title:")
            .position(0, 0)
            .size(100, 25)
            .get(cache),
        TextInput::builder()
            .text(&*title)
            .position(100, 0)
            .size(150, 25)
            .on_user_input({
                let title = title.clone();
                move |new| title.set(new.into())
            })
            .get(cache),
        Button::builder()
            .text("CLOSE")
            .position(title.len() as i32 * 5, 25)
            .on_click(|| nwg::stop_thread_dispatch())
            .get(cache),
    ];
    if title.len() % 2 == 0 {
        v.push(Button::builder()
            .text(format!("{} % 2 = 0", title.len()))
            .position(0, 50)
            .get(cache)
        );
    }
    v.push(Button::builder()
        .text(format!("{} % 2 = 1", title.len()))
        .position(100, 50)
        .enabled(title.len() % 2 == 1)
        .get(cache)
    );
    (title.to_string(), v)
}
