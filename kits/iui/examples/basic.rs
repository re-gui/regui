
use regui_iui::prelude::*;
use controls::*;
use regui::{function_component::Cx, decl_function_component};

#[tokio::main]
async fn main() {
    let ui = UI::init().unwrap();
    run_ui::<Ui>(ui.clone(), &ui).await;
}

decl_function_component!(Ui ui(UI) -> ());

fn ui(ui: &UI, cx: &mut Cx) -> () {
    let counter = cx.use_state(|| 0);

    let button = Button::builder(ui)
        .text(&format!("Counter: {}", counter.get()))
        .on_click({
            let counter = counter.clone();
            move |_btn| {
                counter.set(counter.get() + 1);
            }
        })
        .get(cx);

    let _win = Window::builder(ui)
        .title(&counter.get().to_string())
        .child(button)
        .get(cx);
}