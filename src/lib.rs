
/*
/// Walkaround for <https://github.com/rust-lang/rust/issues/86935> in the
/// [`comp`] macro.
#[allow(type_alias_bounds)]
//pub type ComponentPropsType<C: component::BasicComponent> = <C as component::BasicComponent>::Props;
pub type ComponentPropsType<C: component::BasicComponent> = C::Props;

#[macro_export]
macro_rules! comp {
    ($builder:expr, $Comp:ty { $($id:ident : $y:expr),* }) => {
        ($builder).component::<$Comp>($crate::ComponentPropsType::<$Comp> { $($id:$y),*})
    };
    ($builder:expr, $Comp:ty{ $($id:ident : $y:expr),*, ... }) => {
        ($builder).component::<$Comp>($crate::ComponentPropsType::<$Comp> { $($id:$y),*, ..Default::default() })
    }
}
*/

pub mod component;
pub mod context;
pub mod functional_component;
pub mod kits;

mod callback;

//pub use callback::Callback;