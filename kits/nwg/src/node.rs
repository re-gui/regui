
use super::*;

pub trait NwgNodeTrait {
    type Output;
    #[must_use]
    fn from_parent(&mut self, parent_handle: &nwg::ControlHandle) -> Self::Output;
}

#[derive(Clone)]
pub struct NwgNode<Output>(pub Rc<RefCell<dyn NwgNodeTrait<Output = Output>>>);

impl<Output> PartialEq for NwgNode<Output> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<Output> Deref for NwgNode<Output> {
    type Target = Rc<RefCell<dyn NwgNodeTrait<Output = Output>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}