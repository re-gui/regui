use std::{rc::Rc, ops::Deref};

#[derive(Debug)]
pub struct PtrEqRc<T>(Rc<T>);

impl<T> PtrEqRc<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(value))
    }
    pub fn rc_ref(&self) -> &Rc<T> {
        &self.0
    }
}

impl<T> From<PtrEqRc<T>> for Rc<T> {
    fn from(ptr_eq_rc: PtrEqRc<T>) -> Self {
        ptr_eq_rc.0
    }
}

impl<T> From<Rc<T>> for PtrEqRc<T> {
    fn from(rc: Rc<T>) -> Self {
        Self(rc)
    }
}

impl<T> Deref for PtrEqRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> PartialEq for PtrEqRc<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Clone for PtrEqRc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}