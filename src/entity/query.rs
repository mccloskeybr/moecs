use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::component::Component;

#[derive(Default)]
pub struct Query {
    with_components: Vec<u64>,
    without_components: Vec<u64>,
}

impl Query {
    pub fn with<T: 'static + Component>(mut self) -> Query {
        self.with_components.push(T::property_id());
        self
    }

    pub fn without<T: 'static + Component>(mut self) -> Query {
        self.without_components.push(T::property_id());
        self
    }

    pub(crate) fn get_with_components(&self) -> &Vec<u64> {
        &self.with_components
    }

    pub(crate) fn get_without_components(&self) -> &Vec<u64> {
        &self.without_components
    }
}

#[derive(Clone)]
pub struct QueryResult {
    entity_id: u32,
    component_id_to_component: HashMap<u64, Arc<RwLock<dyn Component>>>,
}

impl QueryResult {
    pub fn new(entity_id: u32) -> Self {
        QueryResult {
            entity_id,
            component_id_to_component: HashMap::new(),
        }
    }

    pub fn entity_id(&self) -> u32 {
        self.entity_id
    }

    pub(crate) fn add_component(
        &mut self,
        component: Arc<RwLock<dyn Component>>,
    ) -> &mut QueryResult {
        self.component_id_to_component.insert(
            component.read().unwrap().self_property_id(),
            component.clone(),
        );
        self
    }

    pub fn get_component<T: 'static + Component>(&self) -> Option<Arc<RwLock<T>>> {
        self.component_id_to_component
            .get(&T::property_id())
            .map(|component| unsafe {
                Arc::from_raw(Arc::into_raw(component.clone()) as *const RwLock<T>)
            })
    }
}
