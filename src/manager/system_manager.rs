use crate::manager::entity_manager::EntityManager;
use crate::system::System;

pub struct SystemManager {
    systems: Vec<Box<dyn System>>,
}

impl Default for SystemManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemManager {
    pub fn new() -> Self {
        SystemManager {
            systems: Vec::new(),
        }
    }

    pub fn register_system<T: 'static + System>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn execute(&self, entity_manager: &mut EntityManager) {
        self.systems.iter().for_each(|system| {
            system.execute(entity_manager);
        })
    }
}
