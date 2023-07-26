
use std::{rc::Rc, cell::RefCell, ops::Deref};

pub use native_windows_gui as nwg;

use regui::{StateFunction, component::{Component, LiveStateComponent}};

//use crate::Callback;

pub mod components;

mod app; pub use app::*;
mod events; pub use events::*;
mod node; pub use node::*;

/// A [`native_windows_gui`] [Common Control](https://learn.microsoft.com/en-us/windows/win32/controls/common-controls-intro)
pub trait WithNwgControlHandle: 'static {
    fn nwg_control_handle(&self) -> &nwg::ControlHandle;

    // TODO the trait name is not very good for the following methods
    // may be they should be moved to a separate trait

    fn position(&self) -> (i32, i32);
    fn size(&self) -> (u32, u32);
}



// TODO this was never used, maybe it should be removed
//impl<Control: WithNwgControlHandle> NwgControlRefData for NCCData<Control> {
//    fn native(&self) -> &dyn WithNwgControlHandle {
//        self.component.as_ref()
//    }
//}
//
//trait NwgControlRefData {
//    #[must_use]
//    fn native(&self) -> &dyn WithNwgControlHandle;
//}

//pub type NwgControlNode = NwgNode<nwg::ControlHandle>;