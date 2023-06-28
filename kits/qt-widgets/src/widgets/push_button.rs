use std::{rc::Rc, cell::RefCell};

use qt_widgets::{qt_core::{QBox, QString, SlotNoArgs}, QPushButton, cpp_core::NullPtr};
use regui::{component::{FunctionsCache, GetFromCache}, StateFunction};

use crate::{widget_props_setters, set_widget_props, update_widget_props};

use super::common::WidgetProps;

pub struct PushButtonProps {
    // TODO ID
    pub widget_props: WidgetProps,
    pub text: String,
    pub on_click: Rc<dyn Fn()>,
}

impl Default for PushButtonProps {
    fn default() -> Self {
        Self {
            text: String::new(),
            widget_props: Default::default(),
            on_click: Rc::new(|| {}),
        }
    }
}

pub struct PushButton {
    qt_button: Rc<QBox<QPushButton>>,
    _click_slot: QBox<SlotNoArgs>,
    on_click_ref: Rc<RefCell<Rc<dyn Fn()>>>,
    props: PushButtonProps,
}

impl PushButton {
    pub fn builder() -> PushButtonBuilder {
        PushButtonBuilder::new()
    }
}

impl StateFunction for PushButton {
    type Input = PushButtonProps;
    type Output = Rc<QBox<QPushButton>>;

    fn build(input: Self::Input) -> (Self::Output, Self) {
        let qt_button = Rc::new(create_push_button(&input));

        let on_click_ref = Rc::new(RefCell::new(input.on_click.clone()));

        let click_slot = {
            let on_click_ref = on_click_ref.clone();
            unsafe {
                let slot = SlotNoArgs::new(NullPtr, move || {
                    // we clone the Rc here so that we can call the function without having a borrow during the call
                    let on_click = on_click_ref.borrow().clone();
                    on_click.as_ref()();
                });
                qt_button.clicked().connect(&slot);
                slot
            }
        };

        (
            qt_button.clone(),
            Self {
                qt_button,
                _click_slot: click_slot,
                on_click_ref,
                props: input,
            },
        )
    }

    fn changed(&mut self, input: Self::Input) -> Self::Output {
        update_push_button(&self.qt_button, &input, &self.props);

        if Rc::ptr_eq(&input.on_click, &self.props.on_click) {
            self.on_click_ref.replace(input.on_click.clone());
        }

        self.props = input;
        self.qt_button.clone()
    }
}

pub struct PushButtonBuilder {
    inner: PushButtonProps,
}

impl PushButtonBuilder {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    widget_props_setters!();

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.inner.text = text.into();
        self
    }

    pub fn on_click(mut self, on_click: impl Fn() + 'static) -> Self {
        self.inner.on_click = Rc::new(on_click);
        self
    }

    pub fn on_click_rc(mut self, on_click: Rc<dyn Fn()>) -> Self {
        self.inner.on_click = on_click;
        self
    }

    pub fn build(self) -> PushButtonProps {
        self.inner
    }
}

impl GetFromCache for PushButtonBuilder {
    type Out = Rc<QBox<QPushButton>>;
    fn get(self, cache: &FunctionsCache) -> Self::Out {
        cache.eval::<PushButton>(self.build())
    }
}

fn create_push_button(props: &PushButtonProps) -> QBox<QPushButton> {
    unsafe {
        let qt_button = QPushButton::new();

        set_widget_props!(qt_button, props);

        qt_button.set_text(&QString::from_std_str(&props.text));

        qt_button
    }
}

fn update_push_button(qt_button: &QPushButton, props: &PushButtonProps, old_props: &PushButtonProps) {
    update_widget_props!(qt_button, props, old_props);

    if props.text != old_props.text {
        unsafe {
            qt_button.set_text(&QString::from_std_str(&props.text));
        }
    }
}