use crate::entity::EntityManager;
use crate::system::{SystemGroup, SystemParamAccessor};

#[derive(Default)]
pub struct SystemManager;

/// The `SystemManager` is simply responsible for executing all of the `System`s in the provided
/// `SystemGroup`.
impl SystemManager {
    pub(crate) fn execute_group(
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
