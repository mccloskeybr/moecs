use std::collections::HashMap;

use crate::entity::EntityManager;
use crate::system::{System, SystemParamAccessor};

#[derive(Default, Clone)]
pub struct SystemGroup {
    system_id_to_system:
        HashMap<u64, fn(entity_manager: &mut EntityManager, params: &SystemParamAccessor)>,
}

impl SystemGroup {
    pub fn register<T: 'static + System>(&mut self) -> &mut SystemGroup {
        let system_id = T::property_id();
        if self.system_id_to_system.contains_key(&system_id) {
            panic!("System {} already registered!", system_id);
        }
        self.system_id_to_system.insert(system_id, T::execute);
        self
    }

    pub fn get_registered_systems(
        &self,
    ) -> &HashMap<u64, fn(entity_manager: &mut EntityManager, params: &SystemParamAccessor)> {
        &self.system_id_to_system
    }
}
