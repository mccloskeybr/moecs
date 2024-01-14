pub use moecs_macros::SystemParam;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::util::PropertyId;

/// A `SystemParam` can be used to pass external data into a `System`. For example, a render
/// canvas, delta time, input processing information, etc.
///
/// Note: All user-defined `SystemParam`s must derive this trait via `#[derive(SystemParam)]`.
pub trait SystemParam: PropertyId {}

/// Stores and provides access to a collection of `SystemParam`s.
#[derive(Default)]
pub struct SystemParamAccessor<'a> {
    param_id_to_param: HashMap<u64, Rc<RefCell<dyn 'a + SystemParam>>>,
}

impl<'a> SystemParamAccessor<'a> {
    pub fn add_param<'b, T: 'a + SystemParam>(
        &'b mut self,
        param: T,
    ) -> &'b mut SystemParamAccessor<'a> {
        self.param_id_to_param
            .insert(param.self_property_id(), Rc::new(RefCell::new(param)));
        self
    }

    pub fn get_param<T: 'a + SystemParam>(&self) -> Option<Rc<RefCell<T>>> {
        self.param_id_to_param
            .get(&T::property_id())
            .map(|param| unsafe { Rc::from_raw(Rc::into_raw(param.clone()) as *const RefCell<T>) })
    }
}
