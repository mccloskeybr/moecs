use std::any::TypeId;

use crate::{EntityBuilder, System, SystemParamAccessor};
use crate::manager::EntityManager;
use crate::manager::SystemManager;

#[derive(Default)]
pub struct Engine {
    entity_manager: EntityManager,
    system_manager: SystemManager,
}

impl Engine {
    pub fn create_entity(&mut self, entity_builder: &mut EntityBuilder) -> u32 {
        self.entity_manager.create_entity(entity_builder)
    }

    pub fn delete_entity(&mut self, entity_id: u32) {
        self.entity_manager.delete_entity(entity_id)
    }

    pub fn register_system<T: 'static + System>(&mut self, system: T) -> TypeId {
        self.system_manager.register_system(system)
    }

    pub fn execute_systems(&mut self, system_ids: Vec<TypeId>, params: &SystemParamAccessor) {
        self.system_manager
            .execute(system_ids, &mut self.entity_manager, params)
    }

    pub fn execute_all_systems(&mut self, params: &SystemParamAccessor) {
        self.system_manager.execute(
            self.system_manager.get_all_registered_system_ids(),
            &mut self.entity_manager,
            params,
        )
    }
}
