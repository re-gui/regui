// TODO a derive macro with auto id check
//pub trait ComponentProps {
//    fn same_id(&self, _other: &Self) -> bool {
//        true
//    }
//}

/// Tells wich component this props is associated with.
///
/// A good pattern is to have a struct, for example `Button` that is the props
/// for the component `ButtonComponent`. In this way, the `Button` struct can
/// be used by a builder to create a `ButtonComponent`: `builder.get(Button { ... })`
pub trait ComponentProps<Ctx> {
    type AssociatedComponent: Component<Ctx, Props = Self>;
}

impl<Ctx> ComponentProps<Ctx> for () {
    type AssociatedComponent = ();
}

pub trait Component<Ctx>: 'static {
    type Props;
    fn build(ctx: &Ctx, props: Self::Props) -> Self;
    fn changed(&mut self, props: Self::Props, ctx: &Ctx);

    /// Tells if the component can be reused with the new props.
    ///
    /// TODO description
    ///
    /// By default, this method returns `true`, so the component is reused.
    fn reuse_with(&self, _props: &Self::Props) -> bool {
        true
    }
}


impl<Ctx> Component<Ctx> for () {
    type Props = ();
    fn build(ctx: &Ctx, props: Self::Props) -> Self {
        ()
    }
    fn changed(&mut self, props: Self::Props, ctx: &Ctx) {}
}