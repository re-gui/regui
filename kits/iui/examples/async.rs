
use regui_iui::prelude::*;
use controls::*;
use regui::{function_component::Cx, decl_function_component};

use tokio::task::spawn_local;

#[tokio::main]
async fn main() {
    let ui = UI::init().unwrap();
    run_ui::<Ui>(ui.clone(), &ui).await;
}

decl_function_component!(Ui ui(UI) -> ());

fn ui(ui: &UI, cx: &mut Cx) -> () {
    let counter = cx.use_state(|| 0);

    cx.use_ref(|| {
        // increment counter after 2 second
        spawn_local({
            let counter = counter.clone();
            async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                counter.set(counter.get() + 42);
            }
        });
    });

    let button = Button::builder(ui)
        .text(&"Increment after 1 second")
        .on_click({
            let counter = counter.clone();
            move |_| {
                let counter = counter.clone();
                spawn_local(async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    counter.set(counter.get() + 1);
                });
            }
        })
        .get(cx);

    let label = Label::builder(ui)
        .text(&format!("Counter: {}", counter.get()))
        .get(cx);

    let vbox = VerticalBox::builder(ui)
        .child(label, LayoutStrategy::Compact)
        .child(button, LayoutStrategy::Compact)
        .get(cx);

    let _win = Window::builder(ui)
        .title(&counter.get().to_string())
        .child(vbox)
        .get(cx);
}