use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::entity::EntityManager;
use crate::system::{System, SystemParamAccessor};
use crate::util::ExecutionMode::{self, *};

type SystemExecuteFn =
    fn(entity_manager: Arc<RwLock<EntityManager>>, params: Arc<SystemParamAccessor>);

/// A group of `System`s, defined by the user. This is useful to group and execute similar
/// `System`s together, and also to provide some freedom in establishing an execution order.
/// E.g., it may be useful to execute a render `System` after all physics calculations have
/// completed.
///
/// The user can define whether a new group should be executed in sequence via
/// `SystemGroup::new_sequential_group()`, or in parallel via
/// `SystemGroup::new_parallel_group()`.
#[derive(Clone)]
pub struct SystemGroup {
    system_id_to_system: HashMap<u64, SystemExecuteFn>,
    execution_mode: ExecutionMode,
}

impl SystemGroup {
    pub fn new_sequential_group() -> Self {
        SystemGroup {
            system_id_to_system: HashMap::new(),
            execution_mode: Sequential,
        }
    }

    pub fn new_parallel_group() -> Self {
        SystemGroup {
            system_id_to_system: HashMap::new(),
            execution_mode: Parallel,
        }
    }

    pub fn register<T: 'static + System>(mut self) -> SystemGroup {
        let system_id = T::property_id();
        if self.system_id_to_system.contains_key(&system_id) {
            panic!("System {} already registered!", system_id);
        }
        self.system_id_to_system.insert(system_id, T::execute);
        self
    }

    pub(crate) fn get_registered_systems(&self) -> &HashMap<u64, SystemExecuteFn> {
        &self.system_id_to_system
    }

    pub(crate) fn get_execution_mode(&self) -> &ExecutionMode {
        &self.execution_mode
    }
}
