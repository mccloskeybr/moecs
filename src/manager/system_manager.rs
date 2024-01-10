use std::collections::HashMap;

use crate::manager::entity_manager::EntityManager;
use crate::system::{System, SystemParamAccessor};

#[derive(Default)]
pub struct SystemManager {
    system_id_to_system: HashMap<u64, Box<dyn 'static + System>>,
}

impl SystemManager {
    pub fn register_system<T: 'static + System>(&mut self, system: T) -> u64 {
        let system_id = system.self_property_id();
        self.system_id_to_system
            .insert(system_id, Box::new(system));
        system_id
    }

    pub fn get_all_registered_system_ids(&self) -> Vec<u64> {
        self.system_id_to_system.keys().copied().collect()
    }

    pub fn execute(
        &self,
        system_ids_to_execute: &[u64],
        entity_manager: &mut EntityManager,
        params: &SystemParamAccessor,
    ) {
        system_ids_to_execute.iter().for_each(|system_id| {
            if let Some(system) = self.system_id_to_system.get(system_id) {
                system.execute(entity_manager, params);
            }
        });
    }
}
