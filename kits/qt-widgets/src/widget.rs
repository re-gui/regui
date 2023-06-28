use std::any::Any;



trait QtWidget: Any { // NOTE: Any is required for downcasting
    fn set_parent(&mut self, parent: &mut dyn QtWidget);

    fn for_each_child(&self, f: &mut dyn FnMut(&dyn QtWidget));

    fn for_each_child_mut(&mut self, f: &mut dyn FnMut(&mut dyn QtWidget));
}

