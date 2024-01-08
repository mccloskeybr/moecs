use std::cell::RefCell;
use std::rc::Rc;

use crate::Component;

#[derive(Default)]
pub struct EntityBuilder {
    components: Vec<Rc<RefCell<dyn Component>>>,
}

impl EntityBuilder {
    pub fn add_component<T: Component>(&mut self, component: T) -> &mut EntityBuilder {
        self.components.push(Rc::new(RefCell::new(component)));
        self
    }

    pub fn get_components(&self) -> &Vec<Rc<RefCell<dyn Component>>> {
        &self.components
    }
}
