use crate::entity::EntityManager;
use crate::system::{SystemGroup, SystemParamAccessor};

#[derive(Default)]
pub struct SystemManager;

impl SystemManager {
    pub fn execute_group(
        &self,
        group: &SystemGroup,
        entity_manager: &mut EntityManager,
        params: &SystemParamAccessor,
    ) {
        group.get_registered_systems().values().for_each(|system| {
            system(entity_manager, params);
        });
    }
}
