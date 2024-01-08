pub use pecs_macros::SystemParam;

use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::manager::EntityManager;

pub trait System: 'static {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn execute(&self, entity_manager: &mut EntityManager, params: &SystemParamAccessor);
}

pub trait SystemParam: 'static {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

pub struct SystemParamAccessor {
    param_id_to_param: HashMap<TypeId, Rc<RefCell<dyn SystemParam>>>,
}

impl Default for SystemParamAccessor {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemParamAccessor {
    pub fn new() -> Self {
        SystemParamAccessor {
            param_id_to_param: HashMap::new(),
        }
    }

    pub fn add_param<T: SystemParam>(&mut self, param: T) -> &mut SystemParamAccessor {
        self.param_id_to_param
            .insert(TypeId::of::<T>(), Rc::new(RefCell::new(param)));
        self
    }

    pub fn get_param<T: SystemParam>(&self) -> Option<Rc<RefCell<T>>> {
        self.param_id_to_param
            .get(&TypeId::of::<T>())
            .map(|param| unsafe { Rc::from_raw(Rc::into_raw(param.clone()) as *const RefCell<T>) })
    }
}
