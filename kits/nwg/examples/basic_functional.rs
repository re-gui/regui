// uncomment the line below to compile this example to a windows executable
// that does not open a console window, but it cannot print to stdout
// #![windows_subsystem = "windows"]


use std::rc::Rc;

use native_windows_gui as nwg;
use regui::{component::{FunctionsCache, GetFromCache, EvalFromCache}, function_component::{State, FunctionComponent}, decl_function_component};
use regui_nwg::{NwgNode, components::{Window, Button, Label, TextInput, ProgressBar}, run_ui, WindowEvent};

fn main() {
    // init nwg
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    // run the ui loop, using the function component `ExampleUi`
    run_ui::<FunctionComponent<ExampleUi>>(());
}

decl_function_component!(ExampleUi example_ui(()) -> ());

fn example_ui(_props: &(), cache: &FunctionsCache, state: &mut State) -> () {
    // prepare the icon of the window. It will be initialized only once, and then reused
    let icon = state.use_state(|| {
        const LOGO_PNG: &[u8] = include_bytes!("logo.png");
        let icon = nwg::Icon::from_bin(LOGO_PNG).expect("Failed to load icon");
        Rc::new(icon)
    });

    // evaluate the ExampleContent component, and get the title and content of the window
    let (title, content) = ExampleContent::eval(cache, 42);

    let _ = Window::builder()
        .title(&title)
        .content(content)
        .on_window_event(|event| {
            match event {
                WindowEvent::CloseRequest => nwg::stop_thread_dispatch(),
                _ => {}
            }
        })
        .icon_opt(if title.len() % 2 == 0 { Some(icon.get().clone()) } else { None })
        .initial_size(300, 150)
        .build()
        .get(cache);
}

decl_function_component!(ExampleContent example_content(i32) -> (String, Vec<NwgNode<nwg::ControlHandle>>));

fn example_content(props: &i32, cache: &FunctionsCache, state: &mut State) -> (String, Vec<NwgNode<nwg::ControlHandle>>) {
    // We declare title as a state, this means it will be initialized only once, and then reused
    // but it can be changed using `title.set(new_title)`, causing the component to re-render
    let title = state.use_state(|| props.to_string());

    // we will put the controls in a vector, and return it at the end
    let mut v = vec![
        Label::builder()
            .text("window title:")
            .position(0, 0)
            .size(100, 25)
            .get(cache),
        TextInput::builder()
            .text(title.get())
            .position(100, 0)
            .size(150, 25)
            .on_user_input({
                let title = title.clone();
                move |new| title.set(new.into())
            })
            .get(cache),
        Button::builder()
            .text("CLOSE")
            .position(title.get().len() as i32 * 5, 25)
            .on_click(|| nwg::stop_thread_dispatch())
            .get(cache),
        ProgressBar::builder()
            .position(0, 75)
            .size(200, 25)
            .value(title.get().len() as f32 * 0.05)
            .get(cache),
    ];
    if title.get().len() % 2 == 0 {
        // we place this button only if the title length is even 
        v.push(Button::builder()
            .text(format!("{} % 2 = 0", title.get().len()))
            .position(0, 50)
            .get(cache)
        );
    }
    v.push(Button::builder()
        .text(format!("{} % 2 = 1", title.get().len()))
        .position(100, 50)
        .enabled(title.get().len() % 2 == 1)
        .get(cache)
    );
    (title.get().to_string(), v)
}
