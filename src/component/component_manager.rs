use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::component::Component;

pub struct ComponentManager<T: Component + ?Sized> {
    entity_id_to_component: HashMap<u32, Arc<RwLock<T>>>,
}

impl<T: Component + ?Sized> ComponentManager<T> {
    pub fn new() -> Self {
        ComponentManager {
            entity_id_to_component: HashMap::new(),
        }
    }

    pub fn register_entity(&mut self, entity_id: &u32, component: Arc<RwLock<T>>) {
        self.entity_id_to_component.insert(*entity_id, component);
    }

    pub fn deregister_entity(&mut self, entity_id: &u32) {
        self.entity_id_to_component.remove(entity_id);
    }

    pub fn get_component_for_entity(&self, entity_id: &u32) -> Option<Arc<RwLock<T>>> {
        self.entity_id_to_component.get(entity_id).cloned()
    }
}
