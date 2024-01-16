use std::collections::HashMap;

use crate::component::ComponentBundle;
use crate::entity::{Query, QueryResult};

/// Caches Entity queries to improve lookup speed.
#[derive(Default)]
pub(crate) struct QueryCache {
    query_cache: HashMap<Query, Vec<QueryResult>>,
}

impl QueryCache {
    pub fn new() -> Self {
        QueryCache {
            query_cache: HashMap::new(),
        }
    }

    /// Checks the cache if a given `Query` is represented. Returns `Some(results)` if yes, `None`
    /// if not.
    pub fn check_cache(&self, query: &Query) -> Option<Vec<QueryResult>> {
        return self.query_cache.get(query).map(|results| results.to_vec());
    }

    /// Adds a particular `Query` and `Results` to the cache.
    ///
    /// Note: will panic if the `Query` is already represented.
    pub fn add_to_cache(&mut self, entry: (Query, Vec<QueryResult>)) {
        if self.query_cache.contains_key(&entry.0) {
            panic!(
                "Query cache already contains the provided Query: {:?}!",
                entry.0
            );
        }
        self.query_cache.insert(entry.0, entry.1);
    }

    pub fn remove_entity_from_cache(&mut self, entity_id: &u32) {
        self.query_cache.iter_mut().for_each(|(_query, results)| {
            for i in 0..results.len() {
                if results[i].entity_id() == *entity_id {
                    results.swap_remove(i);
                    break;
                }
            }
        });
    }

    /// Updates the cache given an Entity and *all* of its `Component`s.
    /// Note that this process grows in cost based on the number of queries in the cache.
    pub fn update_cache(&mut self, entity_id: &u32, all_components: &ComponentBundle) {
        self.remove_entity_from_cache(entity_id);
        if all_components.get_components().is_empty() {
            return;
        }

        self.query_cache.iter_mut().for_each(|(query, results)| {
            if all_components.get_components().keys().all(|component_id| {
                query.get_with_components().contains(component_id)
                    && !query.get_without_components().contains(component_id)
            }) {
                let mut result = QueryResult::new(*entity_id);
                query.get_with_components().iter().for_each(|component_id| {
                    result.add_component(
                        all_components
                            .get_components()
                            .get(component_id)
                            .unwrap()
                            .clone(),
                    );
                });
                results.push(result)
            }
        });
    }
}
