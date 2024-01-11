use std::collections::HashMap;

use crate::entity::EntityManager;
use crate::system::{System, SystemParamAccessor};

#[derive(Default)]
pub struct SystemManager {
    system_id_to_system:
        HashMap<u64, fn(entity_manager: &mut EntityManager, params: &SystemParamAccessor)>,
}

impl SystemManager {
    pub fn register_system<T: 'static + System>(&mut self) -> u64 {
        let system_id = T::property_id();
        if self.system_id_to_system.contains_key(&system_id) {
            panic!("System {} already registered!", system_id);
        }
        self.system_id_to_system.insert(system_id, T::execute);
        system_id
    }

    pub fn deregister_system<T: 'static + System>(&mut self) {
        self.system_id_to_system.remove(&T::property_id());
    }

    pub fn execute(
        &self,
        system_id: &u64,
        entity_manager: &mut EntityManager,
        params: &SystemParamAccessor,
    ) {
        if let Some(system) = self.system_id_to_system.get(system_id) {
            system(entity_manager, params);
        }
    }
}
