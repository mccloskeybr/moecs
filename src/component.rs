pub use pecs_macros::Component;

use crate::util::PropertyId;

pub trait Component: PropertyId {}
