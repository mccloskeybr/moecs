pub use moecs_macros::System;

use crate::entity::EntityManager;
use crate::system::SystemParamAccessor;
use crate::util::PropertyId;

/// A `System` is a process that operates over a subset of entities, generally with known
/// `Component`s. This is how `Component`s are able to interact with each other. External
/// information is able to be passed into the `System` via `SystemParamAccessor`.
///
/// Note: All user-defined `SystemParam`s must derive this trait via `#[derive(System)]`.
pub trait System: PropertyId {
    /// The entry point for any `System`. This is invoked sequentially.
    fn execute(entity_manager: &mut EntityManager, params: &SystemParamAccessor);
}
