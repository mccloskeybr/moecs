use crate::manager::EntityManager;

pub trait System {
    fn execute(&self, entity_manager: &mut EntityManager);
}
