pub use moecs_macros::Component;

use crate::util::PropertyId;

/// A `Component` is simply a bundle of data tied to an `Entity`.
///
/// Note: All user-defined `Component`s must derive this trait via `#[derive(Component)]`.
pub trait Component: PropertyId + Send + Sync {}
