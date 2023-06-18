use std::borrow::Cow;

use native_windows_gui as nwg;

use crate::{Callback, component::{ComponentProps, Component}, kits::nwg_old::{NwgCtx, NwgWidget, NwgChildComponent}};

use super::{NativeCommonComponent, NativeCommonComponentProperties, NwgNativeCommonControl};



impl NwgNativeCommonControl for nwg::Button {
    fn handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
}

pub struct ButtonComponent {
    native: Option<(nwg::ControlHandle, NativeCommonComponent<nwg::Button>)>,
    props: Button,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub id: Option<i32>,
    pub text: Cow<'static, str>,
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    pub on_click: Callback<()>,
    pub enabled: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            id: None,
            text: "".into(),
            position: None,
            size: None,
            on_click: Callback::from(|_| {}),
            enabled: true,
        }
    }
}

impl ComponentProps<NwgCtx> for Button {
    type AssociatedComponent = ButtonComponent;
}

impl Component<NwgCtx> for ButtonComponent {
    type Props = Button;
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
            if props.enabled != self.props.enabled {
                native.component.set_enabled(props.enabled);
            }
            if props.on_click != self.props.on_click {
                // TODO written by copilot, to be checked
                let on_click = props.on_click.clone();
                native.change_handler(Some(move |event, _event_data, _handle| {
                    if event == nwg::Event::OnButtonClick {
                        on_click.call(());
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

// TODO static IMG: &[u8] = include_bytes!("../../../../test.png");

impl NwgChildComponent for ButtonComponent {
    fn set_parent_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) {
        if let Some((parent, _)) = &self.native {
            if parent == &parent_window {
                return;
            }
        }

        // TODO let bitmap2 = nwg::Icon::from_bin(IMG).unwrap();

        let text = self.props.text.clone().into_owned();
        let props = self.props.clone();
        let build = Callback::from(move |window_handle: nwg::ControlHandle| -> nwg::Button {
            let mut button = Default::default();
            let builder = nwg::Button::builder()
                //.icon(Some(&bitmap2))
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

            let builder = if props.enabled {
                builder
            } else {
                builder.enabled(false)
            };

            builder
                .build(&mut button)
                .unwrap();
            button
        });

        let on_click = self.props.on_click.clone();
        let on_event = Some(Callback::from(move |(event, _event_data, _handle)| {
            if event == nwg::Event::OnButtonClick {
                on_click.call(());
            }
        }));

        self.native = Some((
            parent_window,
            NativeCommonComponent::build(
                ctx,
                NativeCommonComponentProperties {
                    parent_window,
                    build,
                    on_event,
                },
            ),
        ));
    }
}

impl NwgWidget for ButtonComponent {
    fn set_parent_and_get_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) -> &nwg::ControlHandle {
        self.set_parent_handle(parent_window, ctx);
        self.native.as_ref().unwrap().1.component.handle()
    }
    fn current_handle(&self) -> Option<&nwg::ControlHandle> {
        self.native.as_ref().map(|(handle, _)| handle)
    }
}