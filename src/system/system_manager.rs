use rayon::prelude::*;
use std::sync::{Arc, RwLock};

use crate::entity::EntityManager;
use crate::system::{SystemGroup, SystemParamAccessor};
use crate::util::ExecutionMode::*;

#[derive(Default)]
pub struct SystemManager;

/// The `SystemManager` is simply responsible for executing all of the `System`s in the provided
/// `SystemGroup`.
impl SystemManager {
    pub(crate) fn execute_group(
        &self,
        group: &SystemGroup,
        entity_manager: Arc<RwLock<EntityManager>>,
        params: Arc<SystemParamAccessor>,
    ) {
        match *group.get_execution_mode() {
            Sequential => {
                group.get_registered_systems().values().for_each(|system| {
                    system(entity_manager.clone(), params.clone());
                });
            }
            Parallel => {
                group
                    .get_registered_systems()
                    .par_iter()
                    .for_each(|(_, system)| {
                        system(entity_manager.clone(), params.clone());
                    });
            }
        }
    }
}
