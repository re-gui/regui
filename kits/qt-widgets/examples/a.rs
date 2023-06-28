use std::{any::Any, ops::Deref, rc::Rc};

use qt_widgets::{QApplication, qt_core::{QTimer, SlotNoArgs, QString, QBox}, cpp_core::NullPtr, QPushButton, QStyle};
use regui::{component::{FunctionsCache, LiveStateComponent, GetFromCache}, function_component::FunctionComponent, decl_function_component, StateFunction, utils::PtrEqRc};

use regui::function_component::State;
use regui_qt_widgets::widgets::PushButton;


fn main() {
    QApplication::init(|app| unsafe {
        let (_value, _component) = LiveStateComponent::<FunctionComponent<Ui>>::build(());

        QApplication::exec()
    });
}

decl_function_component!(Ui ui(()) -> ());

fn ui(_props: &(), cache: &FunctionsCache, state: &mut State) -> () {
    let counter = state.use_state(|| 0);

    let button = PushButton::builder()
        .text(format!("ciao {}", counter.get()))
        .on_click(move || counter.set(counter.get() + 1))
        .min_size(300, 100)
        .get(cache);

    let a = |w: qt_widgets::QOpenGLWidget| unsafe {
        w.set_parent(button.deref());
    };

    //w.mouse

    unsafe {
        button.show();
    }
}