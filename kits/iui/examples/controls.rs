// uncomment the line below to compile this example to a windows executable
// that does not open a console window, but it cannot print to stdout
// #![windows_subsystem = "windows"]

use std::future::Future;
use std::time::Duration;

use regui_iui::prelude::*;
use regui_iui::controls::{Button, Window, VerticalBox, Checkbox, Combobox, Entry, Group, HorizontalBox, HorizontalSeparator, Label, MultilineEntry, Spacer, PasswordEntry};

fn timeout(timeout: Duration, f: impl Future + 'static) {
    tokio::task::spawn_local(async move {
        tokio::time::sleep(timeout).await;
        f.await;
    });
}

#[tokio::main]
async fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");

    run_ui::<Ui>(ui.clone(), &ui).await;
}

decl_function_component!(Ui ui(UI) -> ());

fn ui(ui: &UI, cx: &mut Cx) -> () {

    let counter = cx.use_state(|| 0);

    cx.use_ref(|| {
        timeout(Duration::from_secs(1), {
            let counter = counter.clone();
            async move {
                counter.set(counter.get() + 42);
            }
        });
    });

    let button_1 = Button::builder(ui)
        .text(&format!("Counter: {}", counter.get()))
        .on_click({
            let counter = counter.clone();
            move |_btn| {
                counter.set(counter.get() + 1);
            }
        })
        .get(cx);

    let button_2 = Button::builder(ui)
        .text("Hello")
        .get(cx);

    let slider = controls::Slider::builder(ui)
        .on_changed({
            let counter = counter.clone();
            move |value| {
                counter.set(value);
            }
        })
        .value(counter.get())
        .get(cx);

    let check = Checkbox::builder(ui)
        .text("Check me")
        .on_toggled({
            let counter = counter.clone();
            move |checked| {
                counter.set(if checked { 1 } else { 0 });
            }
        })
        .checked(counter.get() % 2 == 1)
        .get(cx);

    let combo = Combobox::builder(ui)
        .items((0..=100).map(|i| i.to_string()).collect::<Vec<_>>())
        .selected(counter.get() as usize)
        .on_selected({
            let counter = counter.clone();
            move |selected| {
                counter.set(selected as i32);
            }
        })
        .get(cx);

    let hr = HorizontalSeparator::builder(ui)
        .get(cx);

    let entry = Entry::builder(ui)
        .on_changed(|s| println!("Entry changed: {}", s))
        .get(cx);

    let vbox = VerticalBox::builder(ui)
        .child(button_1, LayoutStrategy::Compact)
        .child(button_2, LayoutStrategy::Stretchy)
        .child(slider, LayoutStrategy::Compact)
        .child(check, LayoutStrategy::Compact)
        .child(combo, LayoutStrategy::Compact)
        .child(hr, LayoutStrategy::Compact)
        .child(entry, LayoutStrategy::Compact)
        .get(cx);

    let g = Group::builder(ui)
        .title("Group")
        .child(vbox)
        .get(cx);

    let label = Label::builder(ui)
        .text("Label")
        .get(cx);

    let entry = MultilineEntry::builder(ui)
        .on_changed(|s| {
            println!("MultilineEntry changed: {}", s.len());
        })
        .get(cx);

    let spacer = Spacer::builder(ui).get(cx);

    let pwd = PasswordEntry::builder(ui)
        .get(cx);

    let vbox = VerticalBox::builder(ui)
        .child(label, LayoutStrategy::Compact)
        .child(entry, LayoutStrategy::Stretchy)
        .child(spacer, LayoutStrategy::Stretchy)
        .child(pwd, LayoutStrategy::Compact)
        .get(cx);

    let g2 = Group::builder(ui)
        .child(vbox)
        .title("Group 2")
        .get(cx);

    let hbox = HorizontalBox::builder(ui)
        .child(g, LayoutStrategy::Compact)
        .child(g2, LayoutStrategy::Stretchy)
        .get(cx);

    let _win = Window::builder(ui)
        .title(&format!("{}", counter.get()))
        .initial_size(400, 400)
        .child(hbox)
        .get(cx);
}

