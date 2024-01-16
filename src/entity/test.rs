#![cfg(test)]

use std::sync::{Arc, RwLock};

use crate::component::{Component, ComponentBundle};
use crate::entity::*;
use crate::util::PropertyId;

#[derive(Component)]
struct TestComponent;

#[derive(Component)]
struct OtherTestComponent;

#[test]
fn query_with_success() {
    let query = Query::new().with::<TestComponent>();
    assert_eq!(query.get_with_components().len(), 1);
    assert_eq!(
        query.get_with_components().first().unwrap(),
        &TestComponent::property_id()
    );
}

#[test]
fn query_without_success() {
    let query = Query::new().without::<TestComponent>();
    assert_eq!(query.get_without_components().len(), 1);
    assert_eq!(
        query.get_without_components().first().unwrap(),
        &TestComponent::property_id()
    );
}

#[test]
fn query_result_success() {
    let entity_id = 1;
    let entity_component = Arc::new(RwLock::new(TestComponent));
    let mut result = QueryResult::new(entity_id);
    result.add_component(entity_component.clone());
    assert!(Arc::ptr_eq(
        &result.get_component::<TestComponent>().unwrap(),
        &entity_component
    ));
}

#[test]
fn entity_manager_create_entity_success() {
    let mut manager = EntityManager::new();
    let entity_id = manager.create_entity(ComponentBundle::new().add_component(TestComponent));
    assert!(manager
        .filter(Query::new().with::<TestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));
}

#[test]
fn entity_manager_create_entity_unique_id_success() {
    let mut manager = EntityManager::new();
    let entity_id = manager.create_entity(ComponentBundle::new());
    // Loop an arbitrarily high amount of times to ensure the same id isn't used.
    for _ in 0..100 {
        let other_entity_id = manager.create_entity(ComponentBundle::new());
        assert_ne!(entity_id, other_entity_id);
    }
}

#[test]
fn entity_manager_add_components_to_entity_success() {
    let mut manager = EntityManager::new();
    let entity_id = manager.create_entity(ComponentBundle::new());
    assert!(manager
        .get_all_components_for_entity(&entity_id)
        .get_components()
        .is_empty());

    manager.add_components_to_entity(
        &entity_id,
        ComponentBundle::new().add_component(TestComponent),
    );
    let component_bundle = manager.get_all_components_for_entity(&entity_id);
    assert_eq!(component_bundle.get_components().len(), 1);
    assert!(component_bundle.get_component::<TestComponent>().is_some());
}

#[test]
#[should_panic]
fn entity_manager_add_components_to_entity_already_registered_panic() {
    let mut manager = EntityManager::new();
    let entity_id = manager.create_entity(ComponentBundle::new().add_component(TestComponent));
    manager.add_components_to_entity(
        &entity_id,
        ComponentBundle::new().add_component(TestComponent),
    );
}

#[test]
fn entity_manager_remove_component_from_entity_success() {
    let mut manager = EntityManager::new();
    let entity_id = manager.create_entity(ComponentBundle::new().add_component(TestComponent));
    let component_bundle = manager.get_all_components_for_entity(&entity_id);
    assert_eq!(component_bundle.get_components().len(), 1);
    assert!(component_bundle.get_component::<TestComponent>().is_some());

    manager.remove_component_from_entity::<TestComponent>(&entity_id);
    let component_bundle = manager.get_all_components_for_entity(&entity_id);
    assert!(component_bundle.get_components().is_empty());
    assert!(component_bundle.get_component::<TestComponent>().is_none());
}

#[test]
fn entity_manager_filter_with_success() {
    let mut manager = EntityManager::new();
    let entity_id = manager.create_entity(ComponentBundle::new().add_component(TestComponent));
    assert!(manager
        .filter(Query::new().with::<TestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));
    assert!(manager
        .filter(Query::new().with::<OtherTestComponent>())
        .iter()
        .all(|result| result.entity_id() != entity_id));
}

#[test]
fn entity_manager_filter_without_success() {
    let mut manager = EntityManager::new();
    let entity_id = manager.create_entity(ComponentBundle::new().add_component(TestComponent));
    assert!(manager
        .filter(Query::new().without::<TestComponent>())
        .iter()
        .all(|result| result.entity_id() != entity_id));
    assert!(manager
        .filter(Query::new().without::<OtherTestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));
}

#[test]
fn query_cache_success() {
    let mut cache = QueryCache::new();
    let entity_id = 1;
    let entity_component = Arc::new(RwLock::new(TestComponent));

    let query = Query::new().with::<TestComponent>();
    let mut query_result = QueryResult::new(entity_id);
    query_result.add_component(entity_component.clone());

    cache.add_to_cache((query.clone(), vec![query_result.clone()]));
    let query_result = cache.check_cache(&query).unwrap();
    assert!(Arc::ptr_eq(
        &query_result
            .first()
            .unwrap()
            .get_component::<TestComponent>()
            .unwrap(),
        &entity_component
    ));

    cache.remove_entity_from_cache(&entity_id);
    assert!(cache.check_cache(&query).unwrap().is_empty());
}

#[test]
#[should_panic]
fn query_cache_add_already_existing_query_panics() {
    let mut cache = QueryCache::new();
    let entity_id = 1;
    let entity_component = Arc::new(RwLock::new(TestComponent));

    let query = Query::new().with::<TestComponent>();
    let mut query_result = QueryResult::new(entity_id);
    query_result.add_component(entity_component.clone());

    cache.add_to_cache((query.clone(), vec![query_result.clone()]));
    cache.add_to_cache((query.clone(), vec![query_result.clone()]));
}

#[test]
fn entity_manager_filter_cached_success() {
    let mut manager = EntityManager::new();
    let entity_id = manager.create_entity(ComponentBundle::new().add_component(TestComponent));
    assert!(manager
        .filter(Query::new().with::<TestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));
    // Subsequent runs should use the cache and also not panic.
    assert!(manager
        .filter(Query::new().with::<TestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));
}
