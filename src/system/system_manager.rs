use std::collections::HashMap;

use crate::entity::EntityManager;
use crate::system::{System, SystemParamAccessor};

#[derive(Default)]
pub struct SystemManager {
    system_id_to_system: HashMap<u64, Box<dyn 'static + System>>,
}

impl SystemManager {
    pub fn register_system<T: 'static + System>(&mut self, system: T) -> u64 {
        let system_id = system.self_property_id();
        if self.system_id_to_system.contains_key(&system_id) {
            panic!("System {} already registered!", system_id);
        }
        self.system_id_to_system.insert(system_id, Box::new(system));
        system_id
    }

    pub fn deregister_system(&mut self, system_property_id: &u64) {
        self.system_id_to_system.remove(system_property_id);
    }

    pub fn execute(
        &self,
        system_id: &u64,
        entity_manager: &mut EntityManager,
        params: &SystemParamAccessor,
    ) {
        if let Some(system) = self.system_id_to_system.get(system_id) {
            system.execute(entity_manager, params);
        }
    }
}
