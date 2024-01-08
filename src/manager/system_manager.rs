use std::any::TypeId;
use std::collections::HashMap;

use crate::manager::entity_manager::EntityManager;
use crate::system::{System, SystemParamAccessor};

pub struct SystemManager {
    system_id_to_system: HashMap<TypeId, Box<dyn System>>,
}

impl Default for SystemManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemManager {
    pub fn new() -> Self {
        SystemManager {
            system_id_to_system: HashMap::new(),
        }
    }

    pub fn register_system<T: 'static + System>(&mut self, system: T) -> TypeId {
        let system_id = system.type_id();
        self.system_id_to_system
            .insert(system.type_id(), Box::new(system));
        system_id
    }

    pub fn get_all_registered_system_ids(&self) -> Vec<TypeId> {
        self.system_id_to_system.keys().copied().collect()
    }

    pub fn execute(
        &self,
        system_ids_to_execute: Vec<TypeId>,
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
