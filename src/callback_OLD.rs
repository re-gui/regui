use std::{rc::Rc, fmt::{Debug, Formatter}, cell::RefCell};

/*trait Call<In, Out> {
    fn call(self, input: In) -> Out;
}*/

/// Represents `dyn Fn(...) -> Out`
///
/// This trait is used to ensure that the user uses the
/// [`Callback`] struct in on of the correct forms:
///  - `Callback<dyn Fn(...) -> Out>`.
///  - `Callback<dyn FnMut(...) -> Out>`.
///  - `Callback<dyn FnOnce(...) -> Out>`.
///
/// Implementations are provided in the [`impl_callback`] macro
//pub trait DynFn {}

//pub struct Callback<F: DynFn + ?Sized> {
pub struct Callback<F: ?Sized> {
    callback: Rc<RefCell<F>>,
}

/*impl<F: DynFn> Debug for Callback<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback<({}) -> {}>") // TODO args and return type
    }
}*/

impl<F> Clone for Callback<F> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback.clone(),
        }
    }
}

macro_rules! impl_callback {
    ($( $i:ident: $I:ident ),*) => {
        //impl<Out, $( $I ),*> DynFn for dyn Fn($( $I ),*) -> Out {}
        //impl<Out, $( $I ),*> DynFn for dyn for<'a> Fn($( &'a $I ),*) -> Out {}
        //impl<Out, $( $I ),*> DynFn for dyn FnMut($( $I ),*) -> Out {}
        //impl<Out, $( $I ),*> DynFn for dyn FnOnce($( $I ),*) -> Out {}

        impl<Out, $( $I ),*> Callback<dyn Fn($( $I ),*) -> Out> {
            pub fn call(&self, $( $i: $I ),*) -> Out {
                let cb = self.callback.borrow();
                (cb)($( $i ),*)
            }
        }

        impl<Out, $( $I ),*> Callback<dyn for<'a> Fn($( &'a $I ),*) -> Out> {
            pub fn call_ref(&self, $( $i: & $I ),*) -> Out {
                let cb = self.callback.borrow();
                (cb)($( $i ),*)
            }
        }

        impl<Out, $( $I ),*> Callback<dyn FnMut($( $I ),*) -> Out> {
            pub fn call_mut(&mut self, $( $i: $I ),*) -> Out { // TODO mut not needed?
                let mut cb = self.callback.borrow_mut();
                (cb)($( $i ),*)
            }
        }

        impl<Out, $( $I ),*> Callback<dyn for<'a> FnMut($( &'a $I ),*) -> Out> {
            pub fn call_mut_ref(&mut self, $( $i: & $I ),*) -> Out { // TODO mut not needed?
                let mut cb = self.callback.borrow_mut();
                (cb)($( $i ),*)
            }
        }

        // TODO impl call_once
        /*impl<Out, $( $I ),*> Callback<dyn FnOnce($( $I ),*) -> Out> {
            pub fn call_once(self, $( $i: $I ),*) -> Out {
                todo!();
            }
        }*/

        impl<Out, $( $I ),*> Debug for Callback<dyn Fn($( $I ),*) -> Out> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let args: Vec<&'static str> = vec![$( std::any::type_name::<$I>() ),*];
                let args = args.join(", ");
                write!(f, "Callback<dyn Fn({}){}>", args, fn_out_type_str::<Out>())
            }
        }

        impl<Out, $( $I ),*> Debug for Callback<dyn FnMut($( $I ),*) -> Out> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let args: Vec<&'static str> = vec![$( std::any::type_name::<$I>() ),*];
                let args = args.join(", ");
                write!(f, "Callback<dyn FnMut({}){}>", args, fn_out_type_str::<Out>())
            }
        }

        impl<Out, $( $I ),*> Debug for Callback<dyn FnOnce($( $I ),*) -> Out> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let args: Vec<&'static str> = vec![$( std::any::type_name::<$I>() ),*];
                let args = args.join(", ");
                write!(f, "Callback<dyn FnOnce({}){}>", args, fn_out_type_str::<Out>())
            }
        }

        impl<F, Out, $( $I ),*> From<F> for Callback<dyn Fn($( $I ),*) -> Out>
        where
            F: Fn($( $I ),*) -> Out + 'static,
        {
            fn from(f: F) -> Self {
                Self {
                    callback: Rc::new(RefCell::new(f)),
                }
            }
        }
    }
}

fn fn_out_type_str<Out>() -> String {
    let out = std::any::type_name::<Out>();
    if out == "()" {
        "".into()
    } else {
        format!(" -> {}", out)
    }
}

// implement from 0 to 10 args
impl_callback!();
impl_callback!(i1: I1);
impl_callback!(i1: I1, i2: I2);
impl_callback!(i1: I1, i2: I2, i3: I3);
impl_callback!(i1: I1, i2: I2, i3: I3, i4: I4);
impl_callback!(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5);
impl_callback!(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6);
impl_callback!(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7);
impl_callback!(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8);
impl_callback!(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9);
impl_callback!(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10);

