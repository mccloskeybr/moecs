#![cfg(test)]

use std::sync::{Arc, RwLock};

use crate::component::*;
use crate::util::PropertyId;

#[derive(Component)]
struct TestComponent;

#[test]
fn component_bundle_success() {
    let bundle = ComponentBundle::new().add_component(TestComponent);

    assert_eq!(bundle.get_components().len(), 1);

    let component = bundle
        .get_components()
        .get(&TestComponent::property_id())
        .unwrap();
    assert_eq!(
        component.read().unwrap().self_property_id(),
        TestComponent::property_id()
    );
}

#[test]
#[should_panic]
fn component_bundle_multiple_same_component_panics() {
    ComponentBundle::new()
        .add_component(TestComponent)
        .add_component(TestComponent);
}

#[test]
fn component_manager_success() {
    let mut manager: ComponentManager<TestComponent> = ComponentManager::new();
    let entity_id = 0;
    let component = Arc::new(RwLock::new(TestComponent));

    manager.register_entity(&entity_id, component.clone());
    assert!(Arc::ptr_eq(
        &manager.get_component_for_entity(&entity_id).unwrap(),
        &component
    ));

    manager.deregister_entity(&entity_id);
    assert!(manager.get_component_for_entity(&entity_id).is_none());
}
