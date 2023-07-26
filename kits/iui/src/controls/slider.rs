use std::rc::Rc;

use iui::{UI, prelude::NumericEntry};
use iui::controls::Slider as IuiSlider;
use regui::function_component::ComponentFunction;
use regui::{decl_function_component, function_component::Cx};

use crate::Control;



pub struct SliderProps {
    pub ui: UI,
    pub min: i32,
    pub max: i32,
    pub enabled: bool,
    pub value: Option<i32>,
    pub on_changed: Rc<dyn Fn(i32)>,
}

impl SliderProps {
    pub fn new(ui: &UI) -> Self {
        Self {
            ui: ui.clone(),
            min: 0,
            max: 100,
            enabled: true,
            value: None,
            on_changed: Rc::new(|_slider| {}),
        }
    }
    pub fn min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }
    pub fn max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }
    pub fn on_changed(mut self, on_changed: impl Fn(i32) + 'static) -> Self {
        self.on_changed = Rc::new(on_changed);
        self
    }
    pub fn get(self, cx: &mut Cx) -> Control {
        Slider::eval(cx, self)
    }
}

decl_function_component!(pub Slider slider(SliderProps) -> Control);

impl Slider {
    pub fn builder(ui: &UI) -> SliderProps {
        SliderProps::new(ui)
    }
}

fn slider(props: &SliderProps, cx: &mut Cx) -> Control {
    let slider = cx.use_state(|| IuiSlider::new(&props.ui, props.min, props.max));

    let control = cx.use_state(|| Control::new(slider.get()));

    let mut slider = slider.get();

    slider.on_changed(&props.ui, {
        let on_changed = props.on_changed.clone();
        move |value| {
            on_changed(value);
        }
    });

    if let Some(value) = props.value {
        slider.set_value(&props.ui, value);
    }

    if props.enabled {
        slider.enable(&props.ui);
    } else {
        slider.disable(&props.ui);
    }

    control.get()
}