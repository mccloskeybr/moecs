use crate::manager::entity_manager::EntityManager;

pub trait System {
    fn execute(&self, entity_manager: &mut EntityManager);
}
