use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::component::Component;

/// Represents a collection of `Component`s. Generally used in relation to an Entity (i.e. creating
/// an Entity using the provided `Component`s).
#[derive(Default)]
pub struct ComponentBundle {
    components: HashMap<u64, Arc<RwLock<dyn 'static + Component>>>,
}

impl ComponentBundle {
    pub fn new() -> Self {
        ComponentBundle {
            components: HashMap::new(),
        }
    }

    /// Adds a given `Component` to the bundle.
    ///
    /// Note: will panic if multiple `Component`s of the same time are registered.
    pub fn add_component<T: 'static + Component>(mut self, component: T) -> ComponentBundle {
        if self.components.contains_key(&T::property_id()) {
            panic!(
                "Component {} cannot be registered more than once per Entity!",
                &T::property_id()
            );
        }

        self.components
            .insert(T::property_id(), Arc::new(RwLock::new(component)));
        self
    }

    /// Retrieve a `Component` of the provided type. Returns `Some(component)` if available, `None`
    /// otherwise.
    pub fn get_component<T: 'static + Component>(
        &self,
    ) -> Option<Arc<RwLock<dyn 'static + Component>>> {
        self.components.get(&T::property_id()).cloned()
    }

    pub(crate) fn add_component_arc(
        &mut self,
        component: Arc<RwLock<dyn 'static + Component>>,
    ) -> &ComponentBundle {
        self.components.insert(
            component.read().unwrap().self_property_id(),
            component.clone(),
        );
        self
    }

    pub(crate) fn get_components(&self) -> &HashMap<u64, Arc<RwLock<dyn 'static + Component>>> {
        &self.components
    }
}
