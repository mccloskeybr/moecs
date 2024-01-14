use std::collections::HashMap;

use crate::entity::EntityManager;
use crate::system::{SystemGroup, SystemManager, SystemParamAccessor};

/// Stores and provides access to user-defined `SystemGroup`s.
///
/// The general shape of `SystemGroup`s, as well as the order in which they are invoked, is
/// specifically left to the caller. When deciding which group a `System` should belong to,
/// consider if there are dependencies between Systems that would encourage one to run before or
/// after another.
///
/// Note: When processing a `SystemGroup`, `System`s are invoked sequentially in the order they are
/// registered.
#[derive(Default)]
pub struct Engine {
    entity_manager: EntityManager,
    system_manager: SystemManager,
    next_group_id: u32,
    system_groups: HashMap<u32, SystemGroup>,
}

impl Engine {
    /// Registeres a provided `SystemGroup`. Returns a `u32` representing the `id` of the provided
    /// `SystemGroup`, for reference during execution.
    pub fn register_system_group(&mut self, group: SystemGroup) -> u32 {
        let group_id = self.next_group_id;
        self.next_group_id += 1;
        self.system_groups.insert(group_id, group);
        group_id
    }

    /// Deregistered a `SystemGroup` given its `id`.
    pub fn deregister_system_group(&mut self, group_id: u32) {
        self.system_groups.remove(&group_id);
    }

    /// Executes a `SystemGroup` registered under the provided `group_id`, passing the `SystemParams`
    /// registered in the `SystemParamAccessor`.
    pub fn execute_group(&mut self, group_id: u32, params: &SystemParamAccessor) {
        match self.system_groups.get(&group_id) {
            None => panic!("SystemGroup with id: {} not registered!", group_id),
            Some(group) => {
                self.system_manager
                    .execute_group(group, &mut self.entity_manager, params)
            }
        }
    }
}
