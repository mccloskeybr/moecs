#![cfg(test)]

use std::sync::{Arc, RwLock};

use crate::entity::EntityManager;
use crate::system::*;
use crate::util::ExecutionMode::*;
use crate::util::PropertyId;

#[derive(Debug)]
struct TestSystem;
impl System for TestSystem {
    fn execute(_entity_manager: Arc<RwLock<EntityManager>>, _params: Arc<SystemParamAccessor>) {}
}
impl PropertyId for TestSystem {
    fn property_id() -> u64 {
        123
    }
    fn self_property_id(&self) -> u64 {
        Self::property_id()
    }
}

#[derive(Debug)]
struct TestParam;
impl SystemParam for TestParam {}
impl PropertyId for TestParam {
    fn property_id() -> u64 {
        456
    }
    fn self_property_id(&self) -> u64 {
        Self::property_id()
    }
}

#[test]
fn system_group_sequential_success() {
    let group = SystemGroup::new_sequential_group().register::<TestSystem>();
    assert!(group
        .get_registered_systems()
        .contains_key(&TestSystem::property_id()));
    assert_eq!(group.get_execution_mode(), &Sequential);
}

#[test]
fn system_group_parallel_success() {
    let group = SystemGroup::new_parallel_group().register::<TestSystem>();
    assert!(group
        .get_registered_systems()
        .get(&TestSystem::property_id())
        .is_some());
    assert_eq!(group.get_execution_mode(), &Parallel);
}

#[test]
fn system_param_accessor_success() {
    let accessor = SystemParamAccessor::default().add_param(TestParam);
    assert!(accessor.get_param::<TestParam>().is_some());
}

#[test]
fn system_manager_sequential_success() {
    SystemManager.execute_group(
        &SystemGroup::new_sequential_group().register::<TestSystem>(),
        Arc::new(RwLock::new(EntityManager::default())),
        Arc::new(SystemParamAccessor::default()),
    );
}

#[test]
fn system_manager_parallel_success() {
    SystemManager.execute_group(
        &SystemGroup::new_parallel_group().register::<TestSystem>(),
        Arc::new(RwLock::new(EntityManager::default())),
        Arc::new(SystemParamAccessor::default()),
    );
}
