pub use moecs_macros::Component;

use crate::util::PropertyId;

pub trait Component: PropertyId {}
