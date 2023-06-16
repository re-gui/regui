use std::{borrow::Cow, rc::Rc};

use native_windows_gui as nwg;

use crate::{component::{Component, ComponentProps}, kits::nwg::{NwgCtx, NwgChildComponent}};

#[derive(Debug, Clone, PartialEq)]
pub struct WindowSettings {
    pub title: Cow<'static, str>,
    pub initial_position: Option<(i32, i32)>,
    pub position: Option<(i32, i32)>,
    pub initial_size: Option<(u32, u32)>,
    pub size: Option<(u32, u32)>,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "".into(),
            initial_position: None,
            position: None,
            initial_size: None,
            size: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Window<ContentProps = ()>
where
    ContentProps: ComponentProps<NwgCtx>,
    ContentProps::AssociatedComponent: NwgChildComponent,
{
    // TODO pub id: Option<Id>,
    pub content: ContentProps, // TODO optional
    pub settings: WindowSettings,
    // TODO font etc.
}

impl<ContentProps: 'static> ComponentProps<NwgCtx> for Window<ContentProps>
where
    ContentProps: ComponentProps<NwgCtx> + Clone,
    ContentProps::AssociatedComponent: NwgChildComponent,
{
    type AssociatedComponent = WindowComponent<ContentProps>;
}

pub struct WindowComponent<ContentProps = ()>
where
    ContentProps: ComponentProps<NwgCtx>,
    ContentProps::AssociatedComponent: NwgChildComponent,
{
    native: Rc<nwg::Window>,
    handler: nwg::EventHandler,
    props: Window<ContentProps>,
    content_component: ContentProps::AssociatedComponent,
}

impl<ContentProps: 'static> Component<NwgCtx> for WindowComponent<ContentProps>
where
    ContentProps: ComponentProps<NwgCtx> + Clone,
    ContentProps::AssociatedComponent: NwgChildComponent,
{
    type Props = Window<ContentProps>;
    fn build(ctx: &NwgCtx, props: Self::Props) -> Self {
        let mut window: nwg::Window = Default::default();
        let mut builder = nwg::Window::builder();

        if let Some((x, y)) = props.settings.initial_position {
            builder = builder.position((x, y));
        }

        if let Some((w, h)) = props.settings.position {
            builder = builder.position((w, h));
        }

        if let Some((w, h)) = props.settings.initial_size {
            builder = builder.size((w as i32, h as i32));
        }

        if let Some((w, h)) = props.settings.size {
            builder = builder.size((w as i32, h as i32));
        }

        builder = builder.title(props.settings.title.as_ref());

        builder
            .build(&mut window)
            .unwrap();
        let window = Rc::new(window);
        // TODO with flags window.set_visible(false);

        let handler = nwg::full_bind_event_handler(
            &window.handle,
            {
                let window = Rc::downgrade(&window);
                let ctx = ctx.clone();
                move |evt, _evt_data, handle| {
                    use nwg::Event as E;

                    if let Some(window) = window.upgrade() {
                        if &handle == &window as &nwg::Window {
                            match evt {
                                E::OnWindowClose => {
                                        //nwg::modal_info_message(&events_window.handle, "Goodbye", &format!("Goodbye {}", name_edit.text()));
                                        nwg::modal_info_message(&handle, "Goodbye", "Goodbye");
                                        nwg::stop_thread_dispatch(); // TODO remove
                                    },
                                _ => {}
                            }
                        }
                    }
                }
            }
        );

        let mut component = ContentProps::AssociatedComponent::build(&ctx, props.content.clone());
        component.set_parent_handle(window.handle, &ctx);

        Self {
            native: window,
            handler,
            props,
            content_component: component,
        }
    }

    fn changed(&mut self, props: Self::Props, ctx: &NwgCtx) {
        if self.props.settings.title != props.settings.title {
            //println!("title changed {} -> {}", self.props.settings.title, props.settings.title);
            self.native.set_text(props.settings.title.as_ref());
        }

        if self.props.settings.position != props.settings.position {
            if let Some((x, y)) = props.settings.position {
                self.native.set_position(x, y);
            }
        }

        if self.props.settings.size != props.settings.size {
            if let Some((w, h)) = props.settings.size {
                self.native.set_size(w, h);
            }
        }

        self.content_component.changed(props.content.clone(), ctx);

        self.props = props;
    }

    fn reuse_with(&self, props: &Self::Props) -> bool {
        // TODO self.props.id == props.id
        true
    }
}

impl<ContentProps> Drop for WindowComponent<ContentProps>
where
    ContentProps: ComponentProps<NwgCtx>,
    ContentProps::AssociatedComponent: NwgChildComponent,
{
    fn drop(&mut self) {
        nwg::unbind_event_handler(&self.handler);
    }
}