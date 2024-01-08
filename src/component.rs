pub use pecs_macros::Component;
use std::any::TypeId;

pub trait Component: 'static {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}
