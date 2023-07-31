use std::{rc::Rc, cell::RefCell, ptr::{null, null_mut}, ops::Deref};

use crate::{Control, basic_control_methods};

pub use libui_ffi::uiAreaDrawParams;
use regui::function_component::{Cx, ComponentFunction};

use super::window::WindowProps;

#[derive(Clone)]
pub struct Area {
    area: *mut libui_ffi::uiArea,
    control: Control,
    data: Rc<RefCell<AreaData>>
}

struct AreaData {
    on_draw: Box<Box<dyn Fn(&uiAreaDrawParams)>>,
    handler: Box<RustAreaHandler>,
}

pub struct AreaProps {
    pub enabled: bool,
    pub show: bool,
}

impl From<Area> for Control {
    fn from(w: Area) -> Self {
        w.control
    }
}

basic_control_methods!(Area, AreaProps);

impl Area {
    /// Create a new window.
    pub fn new(
        title: String,
        initial_size: (i32, i32),
    ) -> Self {
        let handler = RustAreaHandler::new(None);
        let mut handler = Box::new(handler);

        let area = unsafe {
            let title = std::ffi::CString::new(title).unwrap();
            libui_ffi::uiNewArea(
                &mut handler.ui_area_handler as *mut libui_ffi::uiAreaHandler,
            )
        };
        let basic_control = Control::new_raw(area as *mut libui_ffi::uiControl);

        Self {
            area,
            control: basic_control,
            data: Rc::new(RefCell::new(AreaData {
                on_draw: Box::new(Box::new(|_| {})),
                handler,
            }))
        }
    }

    pub fn functional() -> AreaProps {
        AreaProps {
            enabled: true,
            show: true,
        }
    }

    pub fn on_draw(&mut self, f: impl Fn(&uiAreaDrawParams) + 'static) {
        let mut data = self.data.borrow_mut();
        data.on_draw = Box::new(Box::new(f));

    }
}

impl AreaProps {
    pub fn eval(self, cx: &mut Cx) -> Area {
        Self::call(self, cx)
    }
}

impl ComponentFunction for AreaProps {
    type Props = Self;
    type Out = Area;
    fn call<'a>(props: Self::Props, cx: &mut Cx) -> Self::Out {
        let area = cx.use_ref(|| Area::new("".into(), (200, 200)));
        let mut data = area.data.borrow_mut();

        area.deref().clone()
    }
}

trait AreaHandler {
}

#[repr(C)]
struct RustAreaHandler {
    ui_area_handler: libui_ffi::uiAreaHandler,
    trait_object: Option<Box<dyn AreaHandler>>,
}

impl RustAreaHandler {
    fn new(trait_object: Option<Box<dyn AreaHandler>>) -> Self {
        Self {
            ui_area_handler: libui_ffi::uiAreaHandler {
                Draw: None,
                MouseEvent: None,
                MouseCrossed: None,
                DragBroken: None,
                KeyEvent: None,
            },
            trait_object,
        }
    }
}