use std::sync::{Arc, RwLock};

use crate::component::Component;

/// Represents a collection of `Component`s. Generally used in relation to an Entity (i.e. creating
/// an Entity using the provided `Component`s).
#[derive(Default)]
pub struct ComponentBundle {
    components: Vec<Arc<RwLock<dyn 'static + Component>>>,
}

impl ComponentBundle {
    pub fn add_component<T: 'static + Component>(&mut self, component: T) -> &mut ComponentBundle {
        self.components.push(Arc::new(RwLock::new(component)));
        self
    }

    pub(crate) fn get_components(&self) -> &Vec<Arc<RwLock<dyn 'static + Component>>> {
        &self.components
    }
}
