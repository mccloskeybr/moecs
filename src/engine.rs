use std::collections::HashMap;

use crate::entity::EntityManager;
use crate::system::{SystemManager, SystemGroup, SystemParamAccessor};

#[derive(Default)]
pub struct Engine {
    entity_manager: EntityManager,
    system_manager: SystemManager,
    next_group_id: u32,
    system_groups: HashMap<u32, SystemGroup>,
}

impl Engine {
    pub fn register_system_group(&mut self, group: SystemGroup) -> u32 {
        let group_id = self.next_group_id;
        self.next_group_id += 1;
        self.system_groups.insert(group_id, group);
        group_id
    }

    pub fn deregister_system_group(&mut self, group_id: u32) {
        self.system_groups.remove(&group_id);
    }

    pub fn execute_group(&mut self, group_id: u32, params: &SystemParamAccessor) {
        match self.system_groups.get(&group_id) {
            None => panic!("System group with id: {} not registered!", group_id),
            Some(group) => self.system_manager.execute_group(group, &mut self.entity_manager, params),
        }
    }
}
