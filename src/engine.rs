use crate::manager::entity_manager::EntityManager;
use crate::manager::system_manager::SystemManager;
use crate::system::System;

pub struct Engine {
    entity_manager: EntityManager,
    system_manager: SystemManager,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            entity_manager: EntityManager::default(),
            system_manager: SystemManager::default(),
        }
    }

    pub fn entity_manager(&mut self) -> &mut EntityManager {
        &mut self.entity_manager
    }

    pub fn register_system<T: 'static + System>(&mut self, system: T) {
        self.system_manager.register_system(system);
    }

    pub fn execute_systems(&mut self) {
        self.system_manager.execute(&mut self.entity_manager);
    }
}
