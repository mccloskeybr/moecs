pub use pecs_macros::System;

use crate::entity::EntityManager;
use crate::system::SystemParamAccessor;
use crate::util::PropertyId;

pub trait System: PropertyId {
    fn execute(&self, entity_manager: &mut EntityManager, params: &SystemParamAccessor);
}

