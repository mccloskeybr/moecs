use std::cell::RefCell;
use std::rc::Rc;

use crate::Component;

pub struct EntityBuilder {
    components: Vec<Rc<RefCell<dyn Component>>>,
}

impl Default for EntityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityBuilder {
    pub fn new() -> Self {
        EntityBuilder {
            components: Vec::new(),
        }
    }

    pub fn add_component<T: Component>(&mut self, component: T) -> &mut EntityBuilder {
        self.components.push(Rc::new(RefCell::new(component)));
        self
    }

    pub fn get_components(&self) -> &Vec<Rc<RefCell<dyn Component>>> {
        &self.components
    }
}
