
use std::{borrow::Cow, rc::Rc};

use native_windows_gui as nwg;

use crate::{component::{ComponentProps, Component}, kits::nwg_old::{NwgCtx, NwgChildComponent, NwgWidget}, Callback};

use super::{NwgNativeCommonControl, NativeCommonComponent, NativeCommonComponentProperties};

impl NwgNativeCommonControl for nwg::TextInput {
    fn handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
}

#[derive(Clone, PartialEq)]
pub struct TextInput {
    pub id: Option<i32>,
    pub text: Cow<'static, str>,
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    pub on_changed: Callback<String>,
    // TODO font etc.
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            id: None,
            text: "".into(),
            position: None,
            size: None,
            on_changed: Callback::from(|_| {}),
        }
    }
}

impl ComponentProps<NwgCtx> for TextInput {
    type AssociatedComponent = TextInputComponent;
}

pub struct TextInputComponent {
    native: Option<(nwg::ControlHandle, NativeCommonComponent<nwg::TextInput>)>,
    props: TextInput,
}

impl Component<NwgCtx> for TextInputComponent {
    type Props = TextInput;
    fn build(_ctx: &NwgCtx, props: Self::Props) -> Self {
        Self {
            native: None,
            props,
        }
    }
    fn changed(&mut self, props: Self::Props, _ctx: &NwgCtx) {
        if let Some((_, native)) = &mut self.native {
            if props.text != self.props.text {
                native.component.set_text(&props.text);
            }

            if props.position != self.props.position {
                if let Some((x, y)) = props.position {
                    native.component.set_position(x, y);
                }
            }

            if props.size != self.props.size {
                if let Some((width, height)) = props.size {
                    native.component.set_size(width, height);
                }
            }

            if props.on_changed != self.props.on_changed {
                // TODO written by copilot, to be checked
                let on_changed = props.on_changed.clone();
                let component = Rc::downgrade(&native.component);
                native.change_handler(Some(move |event, _event_data, _handle| {
                    if event == nwg::Event::OnTextInput {
                        if let Some(component) = component.upgrade() {
                            on_changed.call(component.text());
                        }
                    }
                }));
            }
        }

        self.props = props;
    }
    fn reuse_with(&self, props: &Self::Props) -> bool {
        self.props.id == props.id
    }
}

impl NwgChildComponent for TextInputComponent {
    fn set_parent_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) {
        if let Some((parent, _)) = &self.native {
            if parent == &parent_window {
                return;
            }
        }

        let text = self.props.text.clone().into_owned();
        let props = self.props.clone();
        let build = Callback::from(move |window_handle: nwg::ControlHandle| -> nwg::TextInput {
            let mut label = Default::default();

            let builder = nwg::TextInput::builder()
                .text(&text)
                .parent(&window_handle);

            let builder = if let Some((x, y)) = props.position {
                builder.position((x, y))
            } else {
                builder
            };

            let builder = if let Some((width, height)) = props.size {
                builder.size((width as i32, height as i32))
            } else {
                builder
            };

            builder
                .build(&mut label)
                .unwrap();

            label
        });

        let on_changed = self.props.on_changed.clone();
        let on_event = Some(Callback::from(move |(event, _event_data, _handle)| {
            if event == nwg::Event::OnTextInput {
                // on_changed.call(_handle.text());
                //todo!();
            }
        }));

        self.native = Some((
            parent_window,
            NativeCommonComponent::build(
                ctx,
                NativeCommonComponentProperties {
                    parent_window,
                    build,
                    on_event: on_event,
                },
            ),
        ));
    }
}

impl NwgWidget for TextInputComponent {
    fn set_parent_and_get_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) -> &nwg::ControlHandle {
        self.set_parent_handle(parent_window, ctx);
        self.native.as_ref().unwrap().1.component.handle()
    }
    fn current_handle(&self) -> Option<&nwg::ControlHandle> {
        self.native.as_ref().map(|(_, native)| native.component.handle())
    }
}