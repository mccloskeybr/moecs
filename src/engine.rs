use crate::entity::{EntityBuilder, EntityManager};
use crate::system::{System, SystemManager, SystemParamAccessor};

#[derive(Default)]
pub struct Engine {
    entity_manager: EntityManager,
    system_manager: SystemManager,
}

impl Engine {
    pub fn create_entity(&mut self, entity_builder: &EntityBuilder) -> u32 {
        self.entity_manager.create_entity(entity_builder)
    }

    pub fn delete_entity(&mut self, entity_id: &u32) {
        self.entity_manager.delete_entity(&entity_id)
    }

    pub fn register_system<T: 'static + System>(&mut self, system: T) -> u64 {
        self.system_manager.register_system(system)
    }

    pub fn deregister_systems(&mut self, property_ids: &[u64]) {
        property_ids
            .iter()
            .for_each(|property_id| self.system_manager.deregister_system(property_id));
    }

    pub fn execute_systems(&mut self, system_ids: &[u64], params: &SystemParamAccessor) {
        system_ids.iter().for_each(|system_id| {
            self.system_manager
                .execute(system_id, &mut self.entity_manager, params)
        });
    }
}
