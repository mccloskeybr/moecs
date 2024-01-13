use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::component::{Component, ComponentManager};
use crate::entity::{EntityBuilder, Query, QueryResult};

#[derive(Default)]
pub struct EntityManager {
    next_entity_id: u32,
    entity_id_to_component_ids: HashMap<u32, HashSet<u64>>,
    component_id_to_component_managers: HashMap<u64, Box<ComponentManager<dyn Component>>>,
}

impl EntityManager {
    pub fn create_entity(&mut self, entity_builder: &EntityBuilder) -> u32 {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entity_id_to_component_ids
            .insert(entity_id, HashSet::new());
        entity_builder
            .get_components()
            .iter()
            .for_each(|component| {
                let component_id: u64 = component.read().unwrap().self_property_id();
                self.entity_id_to_component_ids
                    .get_mut(&entity_id)
                    .map(|component_ids| {
                        if component_ids.contains(&component_id) {
                            panic!(
                                "Component {} already registered for entity {}!",
                                component_id, entity_id
                            );
                        }
                        component_ids.insert(component_id)
                    });
                let component_manager = self
                    .component_id_to_component_managers
                    .entry(component_id)
                    .or_insert(Box::new(ComponentManager::new()));
                component_manager.register_entity(&entity_id, component.clone());
            });
        entity_id
    }

    pub fn delete_entity(&mut self, entity_id: &u32) {
        self.entity_id_to_component_ids.remove(entity_id);
        self.component_id_to_component_managers
            .values_mut()
            .for_each(|component_manager| {
                component_manager.deregister_entity(entity_id);
            });
    }

    pub fn remove_component_from_entity<T: 'static + Component>(&mut self, entity_id: &u32) {
        self.entity_id_to_component_ids
            .get_mut(entity_id)
            .map(|component_ids| component_ids.remove(&T::property_id()));
        if let Some(component_manager) = self
            .component_id_to_component_managers
            .get_mut(&T::property_id())
        {
            component_manager.deregister_entity(entity_id);
        }
    }

    pub fn filter(&self, query: &Query) -> Vec<QueryResult> {
        let mut entities: HashSet<u32> = self.entity_id_to_component_ids.keys().copied().collect();
        entities = entities
            .par_iter()
            .filter(|entity_id| {
                query.get_with_components().par_iter().all(|component_id| {
                    self.entity_id_to_component_ids
                        .get(entity_id)
                        .map(|entity_component_ids| entity_component_ids.contains(component_id))
                        .is_some()
                })
            })
            .copied()
            .collect();
        entities = entities
            .par_iter()
            .filter(|entity_id| {
                query.get_without_components().par_iter().all(|component_id| {
                    self.entity_id_to_component_ids
                        .get(entity_id)
                        .map(|entity_component_ids| entity_component_ids.contains(component_id))
                        .is_none()
                })
            })
            .copied()
            .collect();

        let results: Mutex<Vec<QueryResult>> = Mutex::new(Vec::new());
        entities.par_iter().for_each(|entity_id| {
            let mut result = QueryResult::new(*entity_id);
            query.get_with_components().iter().for_each(|component_id| {
                result.add_component(
                    self.component_id_to_component_managers
                        .get(component_id)
                        .unwrap()
                        .get_component_for_entity(entity_id)
                        .unwrap(),
                );
            });
            results.lock().unwrap().push(result);
        });
        let results = results.lock().unwrap();
        results.clone()
    }
}
