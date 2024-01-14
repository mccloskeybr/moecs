#![cfg(test)]

use crate::system::{SystemGroup, SystemParamAccessor};
use crate::Engine;

#[test]
fn engine_register_system_group_unique_id_success() {
    let mut engine = Engine::default();
    let group_id = engine.register_system_group(SystemGroup::new_sequential_group());
    // Loop an arbitrarily high amount of times to ensure the same id isn't used.
    for _ in 0..100 {
        let other_group_id = engine.register_system_group(SystemGroup::new_sequential_group());
        assert_ne!(group_id, other_group_id);
    }
}

#[test]
fn engine_execute_success() {
    let mut engine = Engine::default();
    let group_id = engine.register_system_group(SystemGroup::new_sequential_group());
    engine.execute_group(group_id, SystemParamAccessor::default());
}

#[test]
#[should_panic]
fn engine_execute_unregistered_group_panics() {
    let unregistered_group_id = 100;
    Engine::default().execute_group(unregistered_group_id, SystemParamAccessor::default());
}

#[test]
#[should_panic]
fn engine_deregister_success() {
    let mut engine = Engine::default();
    let group_id = engine.register_system_group(SystemGroup::new_sequential_group());
    engine.deregister_system_group(group_id);
    engine.execute_group(group_id, SystemParamAccessor::default());
}
