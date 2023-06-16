use std::{borrow::Cow, rc::Rc};

use native_windows_gui as nwg;

use crate::{component::{ComponentProps, Component}, kits::nwg::{NwgCtx, NwgChildComponent, NwgWidget}, Callback};

use super::{NativeCommonComponent, NativeCommonComponentProperties, NwgNativeCommonControl};

impl NwgNativeCommonControl for nwg::Label {
    fn handle(&self) -> &nwg::ControlHandle {
        &self.handle
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub id: Option<i32>,
    pub text: Cow<'static, str>,
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    // TODO font etc.
}

impl Default for Label {
    fn default() -> Self {
        Self {
            id: None,
            text: "".into(),
            position: None,
            size: None,
        }
    }
}

impl ComponentProps<NwgCtx> for Label {
    type AssociatedComponent = LabelComponent;
}

pub struct LabelComponent {
    native: Option<(nwg::ControlHandle, NativeCommonComponent<nwg::Label>)>,
    props: Label,
}

impl Component<NwgCtx> for LabelComponent {
    type Props = Label;
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
        }

        self.props = props;
    }
    fn reuse_with(&self, props: &Self::Props) -> bool {
        self.props.id == props.id
    }
}

impl NwgChildComponent for LabelComponent {
    fn set_parent_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) {
        if let Some((parent, _)) = &self.native {
            if parent == &parent_window {
                return;
            }
        }

        let text = self.props.text.clone().into_owned();
        let props = self.props.clone();
        let build = Callback::from(move |window_handle: nwg::ControlHandle| -> nwg::Label {
            let mut label = Default::default();

            let builder = nwg::Label::builder()
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

        self.native = Some((
            parent_window,
            NativeCommonComponent::build(
                ctx,
                NativeCommonComponentProperties {
                    parent_window,
                    build,
                    on_event: None,
                },
            ),
        ));
    }
}

impl NwgWidget for LabelComponent {
    fn set_parent_and_get_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) -> &nwg::ControlHandle {
        self.set_parent_handle(parent_window, ctx);
        self.native.as_ref().unwrap().1.component.handle()
    }
    fn current_handle(&self) -> Option<&nwg::ControlHandle> {
        self.native.as_ref().map(|(_, native)| native.component.handle())
    }
}