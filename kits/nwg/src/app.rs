
use super::*;

pub fn run_ui<UiComponent: Component>(props: UiComponent::Props) {
    let (_out, _component) = LiveStateComponent::<UiComponent>::build(props);
    nwg::dispatch_thread_events();
}