use std::borrow::Cow;



pub struct WidgetProps {
    pub enabled: bool,
    pub accept_drops: bool,
    pub accessible_description: Option<Cow<'static, str>>,
    pub accessible_name: Option<Cow<'static, str>>,
    pub position: Option<(i32, i32)>,
    pub base_size: Option<(i32, i32)>,
    pub min_size: Option<(i32, i32)>,
    pub max_size: Option<(i32, i32)>,
}

impl Default for WidgetProps {
    fn default() -> Self {
        Self {
            enabled: true,
            accept_drops: false,
            accessible_description: None,
            accessible_name: None,
            base_size: None,
            position: None,
            min_size: None,
            max_size: None,
        }
    }
}

#[macro_export]
macro_rules! widget_props_setters {
    () => {
        pub fn enabled(mut self, enabled: bool) -> Self {
            self.inner.widget_props.enabled = enabled;
            self
        }

        pub fn accept_drops(mut self, accept_drops: bool) -> Self {
            self.inner.widget_props.accept_drops = accept_drops;
            self
        }

        pub fn accessible_description(mut self, accessible_description: impl Into<std::borrow::Cow<'static, str>>) -> Self {
            self.inner.widget_props.accessible_description = Some(accessible_description.into());
            self
        }

        pub fn accessible_name(mut self, accessible_name: impl Into<std::borrow::Cow<'static, str>>) -> Self {
            self.inner.widget_props.accessible_name = Some(accessible_name.into());
            self
        }

        pub fn position(mut self, x: i32, y: i32) -> Self {
            self.inner.widget_props.position = Some((x, y));
            self
        }

        pub fn base_size(mut self, w: i32, h: i32) -> Self {
            self.inner.widget_props.base_size = Some((w, h));
            self
        }

        pub fn min_size(mut self, w: i32, h: i32) -> Self {
            self.inner.widget_props.min_size = Some((w, h));
            self
        }

        pub fn max_size(mut self, w: i32, h: i32) -> Self {
            self.inner.widget_props.max_size = Some((w, h));
            self
        }
    };
}

#[macro_export]
macro_rules! set_widget_props {
    ($qt_button:ident, $props:ident) => {
        $qt_button.set_enabled($props.widget_props.enabled);

        $qt_button.set_accept_drops($props.widget_props.accept_drops);

        if let Some(accessible_description) = &$props.widget_props.accessible_description {
            $qt_button.set_accessible_description(&QString::from_std_str(accessible_description));
        }

        if let Some(accessible_name) = &$props.widget_props.accessible_name {
            $qt_button.set_accessible_name(&QString::from_std_str(accessible_name));
        }

        if let Some((x, y)) = $props.widget_props.position {
            $qt_button.move_2a(x, y);
        }

        if let Some((basew, baseh)) = $props.widget_props.base_size {
            $qt_button.set_base_size_2a(basew, baseh)
        }

        if let Some((minw, minh)) = $props.widget_props.min_size {
            $qt_button.set_minimum_size_2a(minw, minh)
        }

        if let Some((maxw, maxh)) = $props.widget_props.max_size {
            $qt_button.set_maximum_size_2a(maxw, maxh)
        }
    };
}

#[macro_export]
macro_rules! update_widget_props {
    ($qt_button:ident, $props:ident, $old_props:ident) => {
        if $props.widget_props.enabled != $old_props.widget_props.enabled {
            unsafe { $qt_button.set_enabled($props.widget_props.enabled); }
        }

        if $props.widget_props.accept_drops != $old_props.widget_props.accept_drops {
            unsafe { $qt_button.set_accept_drops($props.widget_props.accept_drops); }
        }

        if $props.widget_props.accessible_description != $old_props.widget_props.accessible_description {
            if let Some(accessible_description) = &$props.widget_props.accessible_description {
                unsafe { $qt_button.set_accessible_description(&QString::from_std_str(accessible_description)); }
            } else {
                unsafe { $qt_button.set_accessible_description(&QString::new()); }
            }
        }

        if $props.widget_props.accessible_name != $old_props.widget_props.accessible_name {
            if let Some(accessible_name) = &$props.widget_props.accessible_name {
                unsafe { $qt_button.set_accessible_name(&QString::from_std_str(accessible_name)); }
            } else {
                unsafe { $qt_button.set_accessible_name(&QString::new()); }
            }
        }

        if $props.widget_props.position != $old_props.widget_props.position {
            if let Some((x, y)) = $props.widget_props.position {
                unsafe { $qt_button.move_2a(x, y); }
            }
        }

        if $props.widget_props.base_size != $old_props.widget_props.base_size {
            if let Some((basew, baseh)) = $props.widget_props.base_size {
                unsafe { $qt_button.set_base_size_2a(basew, baseh) }
            }
        }

        if $props.widget_props.min_size != $old_props.widget_props.min_size {
            if let Some((minw, minh)) = $props.widget_props.min_size {
                unsafe { $qt_button.set_minimum_size_2a(minw, minh) }
            }
        }

        if $props.widget_props.max_size != $old_props.widget_props.max_size {
            if let Some((maxw, maxh)) = $props.widget_props.max_size {
                unsafe { $qt_button.set_maximum_size_2a(maxw, maxh) }
            }
        }
    };
}