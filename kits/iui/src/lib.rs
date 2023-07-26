use std::rc::Rc;

pub mod controls;

pub use iui;
use iui::UI;
pub use regui;
use regui::{component::{Component, LiveStateComponent}, StateFunction, function_component::{ComponentFunction, FunctionComponent}};
pub use tokio;

pub mod prelude {
    pub use crate::run_ui;
    pub use crate::controls;
    pub use crate::iui::prelude::LayoutStrategy;
    pub use regui::decl_function_component;
    pub use iui::UI;
    pub use regui::function_component::{State, FunctionComponent, Cx};
}

#[derive(Clone)]
pub struct Control {
    pub control: Rc<iui::controls::Control>,
}

impl PartialEq for Control {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.control, &other.control)
    }
}

impl Control {
    pub fn new<T>(component: T) -> Self
    where
        T: Into<iui::controls::Control>,
    {
        Self {
            control: Rc::new(component.into()),
        }
    }
}

pub async fn run_ui<F: ComponentFunction>(props: F::Props, ui: &UI) {
    run_ui_component::<FunctionComponent<F>>(props, ui).await
}

pub async fn run_ui_component<UiComponent: Component>(props: UiComponent::Props, ui: &UI) {
    let local = tokio::task::LocalSet::new();
    local.run_until(async move {
        //tokio::task::spawn_local
        let (
            _out,
            _component
        ) = LiveStateComponent::<UiComponent>::build(props);

        //ui.main();

        let mut el = ui.event_loop();
        loop {
            if !el.next_tick(ui) {
                break;
            }
            tokio::task::yield_now().await;
        }
    }).await;
}