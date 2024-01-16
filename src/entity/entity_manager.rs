use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

use crate::component::{Component, ComponentBundle, ComponentManager};
use crate::entity::{Query, QueryCache, QueryResult};

/// Drives all actions on Entities (as collections of `Component`s).
#[derive(Default)]
pub struct EntityManager {
    next_entity_id: u32,
    entity_id_to_component_ids: HashMap<u32, HashSet<u64>>,
    component_id_to_component_managers: HashMap<u64, Box<ComponentManager<dyn Component>>>,
    query_cache: RwLock<QueryCache>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            next_entity_id: 0,
            entity_id_to_component_ids: HashMap::new(),
            component_id_to_component_managers: HashMap::new(),
            query_cache: RwLock::new(QueryCache::new()),
        }
    }

    /// Creates a new Entity, and registers all of the provided `Component`s under that Entity.
    /// Returns a `u32` representing that Entity's id.
    ///
    /// Will panic if registering multiple `Component`s of the same type is attempted.
    pub fn create_entity(&mut self, components: ComponentBundle) -> u32 {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entity_id_to_component_ids
            .insert(entity_id, HashSet::new());
        self.add_components_to_entity(&entity_id, components);
        entity_id
    }

    /// Deletes an Entity given its `entity_id`. Removes / deregisters all `Component`s associated
    /// with that Entity.
    pub fn delete_entity(&mut self, entity_id: &u32) {
        self.entity_id_to_component_ids.remove(entity_id);
        self.component_id_to_component_managers
            .values_mut()
            .for_each(|component_manager| {
                component_manager.deregister_entity(entity_id);
            });
        self.query_cache
            .write()
            .unwrap()
            .remove_entity_from_cache(entity_id);
    }

    /// Adds all specified `Component`s to the Entity with the associated `entity_id`.
    ///
    /// Will panic if registering multiple `Component`s of the same type is attempted.
    pub fn add_components_to_entity(&mut self, entity_id: &u32, components: ComponentBundle) {
        components
            .get_components()
            .iter()
            .for_each(|(component_id, component)| {
                self.entity_id_to_component_ids
                    .get_mut(entity_id)
                    .map(|component_ids| {
                        if component_ids.contains(component_id) {
                            panic!(
                                "Component {} already registered for entity {}!",
                                component_id, entity_id
                            );
                        }
                        component_ids.insert(*component_id)
                    });
                let component_manager = self
                    .component_id_to_component_managers
                    .entry(*component_id)
                    .or_insert(Box::new(ComponentManager::new()));
                component_manager.register_entity(entity_id, component.clone());
            });
        self.query_cache
            .write()
            .unwrap()
            .update_cache(entity_id, &self.get_all_components_for_entity(entity_id));
    }

    /// Removes the specified `Component` from the Entity with the associated `entity_id`.
    ///
    /// Will panic if the provided `entity_id` is not registered.
    pub fn remove_component_from_entity<T: 'static + Component>(&mut self, entity_id: &u32) {
        if !self.entity_id_to_component_ids.contains_key(entity_id) {
            panic!("Entity: {} does not exist!", *entity_id);
        }

        self.entity_id_to_component_ids
            .get_mut(entity_id)
            .map(|component_ids| component_ids.remove(&T::property_id()));
        if let Some(component_manager) = self
            .component_id_to_component_managers
            .get_mut(&T::property_id())
        {
            component_manager.deregister_entity(entity_id);
        }
        self.query_cache
            .write()
            .unwrap()
            .update_cache(entity_id, &self.get_all_components_for_entity(entity_id));
    }

    /// Retrieves all of the `Component`s registered for a given `entity_id`.
    pub fn get_all_components_for_entity(&self, entity_id: &u32) -> ComponentBundle {
        let mut component_bundle = ComponentBundle::new();
        self.entity_id_to_component_ids
            .get(entity_id)
            .iter()
            .for_each(|component_ids| {
                component_ids.iter().for_each(|component_id| {
                    component_bundle.add_component_arc(
                        self.component_id_to_component_managers
                            .get(component_id)
                            .unwrap()
                            .get_component_for_entity(entity_id)
                            .unwrap()
                            .clone(),
                    );
                });
            });
        component_bundle
    }

    /// Executes a `Query` over all Entities. Returns a vector of `QueryResult`s.
    /// This is generally how one finds all Entities with a specific `Component` type, for example.
    /// Queries execution is parallelized, and results are automatically cached to improve future
    /// queries of the same type.
    pub fn filter(&self, query: Query) -> Vec<QueryResult> {
        if let Some(query_results) = self.query_cache.read().unwrap().check_cache(&query) {
            return query_results;
        }

        let mut entities: HashSet<u32> = self.entity_id_to_component_ids.keys().copied().collect();
        entities = entities
            .par_iter()
            .filter(|entity_id| {
                query.get_with_components().iter().all(|component_id| {
                    self.entity_id_to_component_ids
                        .get(entity_id)
                        .map(|entity_component_ids| entity_component_ids.contains(component_id))
                        .unwrap()
                })
            })
            .copied()
            .collect();
        entities = entities
            .par_iter()
            .filter(|entity_id| {
                query.get_without_components().iter().all(|component_id| {
                    !self
                        .entity_id_to_component_ids
                        .get(entity_id)
                        .map(|entity_component_ids| entity_component_ids.contains(component_id))
                        .unwrap()
                })
            })
            .copied()
            .collect();

        let mut results: Vec<QueryResult> = Vec::new();
        entities.iter().for_each(|entity_id| {
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
            results.push(result);
        });

        self.query_cache
            .write()
            .unwrap()
            .add_to_cache((query, results.to_vec()));
        results.to_vec()
    }
}
