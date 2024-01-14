#![cfg(test)]

use std::sync::{Arc, RwLock};

use crate::component::{Component, ComponentBundle};
use crate::entity::*;
use crate::util::PropertyId;

#[derive(Debug)]
struct TestComponent;
impl PropertyId for TestComponent {
    fn property_id() -> u64 {
        123
    }
    fn self_property_id(&self) -> u64 {
        Self::property_id()
    }
}
impl Component for TestComponent {}

#[derive(Debug)]
struct OtherTestComponent;
impl PropertyId for OtherTestComponent {
    fn property_id() -> u64 {
        456
    }
    fn self_property_id(&self) -> u64 {
        Self::property_id()
    }
}
impl Component for OtherTestComponent {}

#[test]
fn query_with_success() {
    let query = Query::default().with::<TestComponent>();
    assert_eq!(query.get_with_components().len(), 1);
    assert_eq!(
        query.get_with_components().first().unwrap(),
        &TestComponent::property_id()
    );
}

#[test]
fn query_without_success() {
    let query = Query::default().without::<TestComponent>();
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
    let mut manager = EntityManager::default();
    let entity_id = manager.create_entity(ComponentBundle::default().add_component(TestComponent));
    assert!(manager
        .filter(Query::default().with::<TestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));
}

#[test]
fn entity_manager_create_entity_unique_id_success() {
    let mut manager = EntityManager::default();
    let entity_id = manager.create_entity(ComponentBundle::default());
    // Loop an arbitrarily high amount of times to ensure the same id isn't used.
    for _ in 0..100 {
        let other_entity_id = manager.create_entity(ComponentBundle::default());
        assert_ne!(entity_id, other_entity_id);
    }
}

#[test]
#[should_panic]
fn entity_manager_create_entity_register_multiple_same_component_panic() {
    let mut manager = EntityManager::default();
    manager.create_entity(
        ComponentBundle::default()
            .add_component(TestComponent)
            .add_component(TestComponent),
    );
}

#[test]
fn entity_manager_add_components_to_entity_success() {
    let mut manager = EntityManager::default();
    let entity_id = manager.create_entity(ComponentBundle::default());
    assert!(manager
        .filter(Query::default().with::<TestComponent>())
        .iter()
        .all(|result| result.entity_id() != entity_id));

    manager.add_components_to_entity(
        &entity_id,
        ComponentBundle::default().add_component(TestComponent),
    );
    assert!(manager
        .filter(Query::default().with::<TestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));
}

#[test]
#[should_panic]
fn entity_manager_add_components_to_entity_multiple_same_component_panic() {
    let mut manager = EntityManager::default();
    let entity_id = manager.create_entity(ComponentBundle::default());
    manager.add_components_to_entity(
        &entity_id,
        ComponentBundle::default()
            .add_component(TestComponent)
            .add_component(TestComponent),
    );
}

#[test]
fn entity_manager_remove_component_from_entity_success() {
    let mut manager = EntityManager::default();
    let entity_id = manager.create_entity(ComponentBundle::default().add_component(TestComponent));
    assert!(manager
        .filter(Query::default().with::<TestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));

    manager.remove_component_from_entity::<TestComponent>(&entity_id);
    assert!(manager
        .filter(Query::default().with::<TestComponent>())
        .iter()
        .all(|result| result.entity_id() != entity_id));
}

#[test]
fn entity_manager_filter_with_success() {
    let mut manager = EntityManager::default();
    let entity_id = manager.create_entity(ComponentBundle::default().add_component(TestComponent));
    assert!(manager
        .filter(Query::default().with::<TestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));
    assert!(manager
        .filter(Query::default().with::<OtherTestComponent>())
        .iter()
        .all(|result| result.entity_id() != entity_id));
}

#[test]
fn entity_manager_filter_without_success() {
    let mut manager = EntityManager::default();
    let entity_id = manager.create_entity(ComponentBundle::default().add_component(TestComponent));
    assert!(manager
        .filter(Query::default().without::<TestComponent>())
        .iter()
        .all(|result| result.entity_id() != entity_id));
    assert!(manager
        .filter(Query::default().without::<OtherTestComponent>())
        .iter()
        .any(|result| result.entity_id() == entity_id));
}
