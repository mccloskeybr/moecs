#![cfg(test)]

use std::sync::{Arc, RwLock};

use crate::entity::EntityManager;
use crate::system::*;
use crate::util::ExecutionMode::*;
use crate::util::PropertyId;

#[derive(System)]
struct TestSystem;
impl System for TestSystem {
    fn execute(_entity_manager: Arc<RwLock<EntityManager>>, _params: Arc<SystemParamAccessor>) {}
}

#[derive(SystemParam)]
struct TestParam;

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
    let accessor = SystemParamAccessor::new().add_param(TestParam);
    assert!(accessor.get_param::<TestParam>().is_some());
}

#[test]
fn system_manager_sequential_success() {
    SystemManager.execute_group(
        &SystemGroup::new_sequential_group().register::<TestSystem>(),
        Arc::new(RwLock::new(EntityManager::new())),
        Arc::new(SystemParamAccessor::new()),
    );
}

#[test]
fn system_manager_parallel_success() {
    SystemManager.execute_group(
        &SystemGroup::new_parallel_group().register::<TestSystem>(),
        Arc::new(RwLock::new(EntityManager::new())),
        Arc::new(SystemParamAccessor::new()),
    );
}
