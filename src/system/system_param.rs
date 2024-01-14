pub use moecs_macros::SystemParam;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::util::PropertyId;

/// A `SystemParam` can be used to pass external data into a `System`. For example, a render
/// canvas, delta time, input processing information, etc.
///
/// Note: All user-defined `SystemParam`s must derive this trait via `#[derive(SystemParam)]`.
pub trait SystemParam: PropertyId {}

/// Stores and provides access to a collection of `SystemParam`s.
#[derive(Default)]
pub struct SystemParamAccessor<'a> {
    param_id_to_param: HashMap<u64, Arc<RwLock<dyn 'a + SystemParam>>>,
}

unsafe impl Send for SystemParamAccessor<'_> {}
unsafe impl Sync for SystemParamAccessor<'_> {}

impl<'a> SystemParamAccessor<'a> {
    pub fn add_param<T: 'a + SystemParam>(
        mut self,
        param: T,
    ) -> SystemParamAccessor<'a> {
        self.param_id_to_param
            .insert(param.self_property_id(), Arc::new(RwLock::new(param)));
        self
    }

    pub fn get_param<T: 'a + SystemParam>(&self) -> Option<Arc<RwLock<T>>> {
        self.param_id_to_param
            .get(&T::property_id())
            .map(|param| unsafe { Arc::from_raw(Arc::into_raw(param.clone()) as *const RwLock<T>) })
    }
}
