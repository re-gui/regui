
#[macro_export]
macro_rules! basic_control_methods {
    ($s:ty, $props:ty) => {
        impl $s {
            pub fn control(&self) -> &Control {
                &self.control
            }
            pub fn show(&mut self, show: bool) {
                unsafe {
                    if show {
                        libui_ffi::uiControlShow(self.control.control_ptr());
                    } else {
                        libui_ffi::uiControlHide(self.control.control_ptr());
                    }
                }
            }
            pub fn enable(&mut self, enable: bool) {
                unsafe {
                    if enable {
                        libui_ffi::uiControlEnable(self.control.control_ptr());
                    } else {
                        libui_ffi::uiControlDisable(self.control.control_ptr());
                    }
                }
            }
            pub fn enabled(&self) -> bool {
                unsafe {
                    libui_ffi::uiControlEnabled(self.control.control_ptr()) != 0
                }
            }
        }
        impl PartialEq for $s {
            fn eq(&self, other: &Self) -> bool {
                self.control == other.control
            }
        }
        impl $props {
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.enabled = enabled;
                self
            }
            pub fn show(mut self, show: bool) -> Self {
                self.show = show;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! control_with_child_methods {
    ($s:ty, $props:ty) => {
        impl $s {
            pub fn set_child(&mut self, child: Control) {
                let mut data = self.data.borrow_mut();
                if Some(&child) != data.child.as_ref() {
                    unsafe {
                        libui_ffi::uiWindowSetChild(self.w, child.control_ptr());
                    }
                }
                data.child = Some(child);
            }
            pub fn remove_child(&mut self) {
                let mut data = self.data.borrow_mut();
                if let Some(_) = data.child {
                    unsafe {
                        libui_ffi::uiWindowSetChild(self.w, std::ptr::null_mut());
                    }
                }
                data.child = None;
            }
            pub fn child(&self) -> Option<Control> {
                let data = self.data.borrow();
                data.child.clone()
            }
        }
        impl $props {
            pub fn child<C: Into<Control>>(mut self, child: C) -> Self {
                self.child = Some(child.into());
                self
            }
        }
    };
}