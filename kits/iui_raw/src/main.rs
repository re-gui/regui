use std::{rc::Rc, cell::RefCell};

use iui_raw::{init, run_ui, controls::{window::{WindowProps, Window}, area::Area}};
use regui::{component::{LiveStateComponent, Component}, StateFunction, function_component::{StateVeriablesManager, ComponentFunction, Cx}};

fn main() {
    println!("Hello, world!");

    init();

    //let w = Window::new();
    //w.show();

    //iui_raw::main();

    //Window::functional()
    //    .title("Hello, world!");

    run_ui::<Ui>(());
}

struct Ui;
impl ComponentFunction for Ui {
    type Props = ();
    type Out = ();
    fn call<'a>(_props: Self::Props, cx: &mut Cx) -> Self::Out {

        let _ = Window::functional()
            //.title("Hello, world!")
            //.child(area)
            .eval(cx);
    }
}

