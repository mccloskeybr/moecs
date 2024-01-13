use std::sync::{Arc, RwLock};

use crate::component::Component;

#[derive(Default)]
pub struct EntityBuilder {
    components: Vec<Arc<RwLock<dyn 'static + Component>>>,
}

impl EntityBuilder {
    pub fn add_component<T: 'static + Component>(&mut self, component: T) -> &mut EntityBuilder {
        self.components.push(Arc::new(RwLock::new(component)));
        self
    }

    pub fn get_components(&self) -> &Vec<Arc<RwLock<dyn 'static + Component>>> {
        &self.components
    }
}
