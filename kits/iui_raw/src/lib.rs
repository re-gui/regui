use std::{rc::Rc, cell::RefCell};

use raw_window_handle::{Win32WindowHandle, WindowsDisplayHandle, HasRawWindowHandle, HasRawDisplayHandle};
use regui::{function_component::{Cx, ComponentFunction, FunctionComponent}, decl_function_component, component::{Component, LiveStateComponent}, StateFunction};

pub mod controls;



pub fn init() {
    unsafe {
        let mut options: libui_ffi::uiInitOptions = std::mem::zeroed();
        libui_ffi::uiInit(&mut options);
    }
}

pub fn main() {
    unsafe {
        libui_ffi::uiMain();
    }
}

pub /*async*/ fn run_ui<F: ComponentFunction>(props: F::Props)
where
    F::Props: Clone,
{
    run_ui_component::<FunctionComponent<F>>(props)//.await
}

pub /*async*/ fn run_ui_component<UiComponent: Component>(props: UiComponent::Props) {
    /*let local = tokio::task::LocalSet::new();
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
    }).await;*/
    let (
        _out,
        _component
    ) = LiveStateComponent::<UiComponent>::build(props);
    main();
}

#[derive(Debug, PartialEq, Eq)]
struct ControlInner {
    control: *mut libui_ffi::uiControl,
}

impl Drop for ControlInner {
    fn drop(&mut self) {
        println!("drop control: {:?}", self.control);
        unsafe {
            libui_ffi::uiControlDestroy(self.control);
        }
    }
}

impl ControlInner {
    pub fn new(control: *mut libui_ffi::uiControl) -> Self {
        Self {
            control
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Control {
    inner: Rc<ControlInner>,
}

impl Control {
    pub fn new_raw(control: *mut libui_ffi::uiControl) -> Self {
        Self {
            inner: Rc::new(ControlInner::new(control))
        }
    }
    pub fn control_ptr(&self) -> *mut libui_ffi::uiControl {
        self.inner.control
    }
    pub fn os_handle(&self) -> raw_window_handle::RawWindowHandle {
        // TODO check os
        let h = unsafe {
            libui_ffi::uiControlHandle(self.inner.control)
        };
        let mut handle = Win32WindowHandle::empty();
        handle.hwnd = h as *mut std::ffi::c_void;
        raw_window_handle::RawWindowHandle::Win32(handle)
    }
    pub fn os_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        // TODO check os
        raw_window_handle::RawDisplayHandle::Windows(WindowsDisplayHandle::empty())
    }
}

unsafe impl HasRawWindowHandle for Control {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.os_handle()
    }
}

unsafe impl HasRawDisplayHandle for Control {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.os_display_handle()
    }
}

