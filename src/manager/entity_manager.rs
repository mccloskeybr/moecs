use std::any::TypeId;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::component::Component;
use crate::manager::component_manager::ComponentManager;

pub struct EntityManager {
    next_entity_id: u32,
    entity_id_to_component_ids: HashMap<u32, HashSet<TypeId>>,
    component_id_to_component_managers: HashMap<TypeId, Box<ComponentManager<dyn Component>>>,
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! get_components_from_entity {
    ( $name:ident $( $component:ident ),+ ) => {
        #[allow(unused_parens)]
        pub fn $name<$($component: Component),+> (
            &self,
            entity_id: &u32,
        ) -> ($(Option<Rc<RefCell<$component>>>),+)
        {
            ($(
                self.component_id_to_component_managers
                    .get(&TypeId::of::<$component>())
                    .map(|component_manager| {
                        component_manager
                            .get_component_for_entity(entity_id)
                            .map(|component| unsafe {
                                Rc::from_raw(Rc::into_raw(component) as *const RefCell<$component>)
                            })
                    }).unwrap_or(None)
            ),+)
        }
    };
}

macro_rules! filter_entities_with_components {
    ( $name:ident $( $component:ident ),+ ) => {
        pub fn $name<$($component: Component),+> (&self) -> Vec<u32> {
            let mut matches: HashSet<u32> = self.entity_id_to_component_ids.keys().copied().collect();
            $(
                matches = matches
                    .intersection(
                        &self
                            .component_id_to_component_managers
                            .get(&TypeId::of::<$component>())
                            .map_or(HashSet::new(), |component_manager|
                                component_manager.get_all_registered_entities()
                            ),
                    )
                    .copied()
                    .collect();
            )+
            matches.iter().copied().collect()
        }
    };
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            next_entity_id: 0,
            entity_id_to_component_ids: HashMap::new(),
            component_id_to_component_managers: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> u32 {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entity_id_to_component_ids
            .insert(entity_id, HashSet::new());
        println!("Entity created: {:?}.", entity_id);
        entity_id
    }

    pub fn delete_entity(&mut self, entity_id: u32) {
        self.entity_id_to_component_ids.remove(&entity_id);
        self.component_id_to_component_managers
            .values_mut()
            .for_each(|component_manager| {
                component_manager.deregister_entity(entity_id);
            });
        println!("Entity deleted: {:?}.", entity_id);
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: u32,
        component: T,
    ) {
        let component_id: TypeId = TypeId::of::<T>();
        match self.entity_id_to_component_ids.get_mut(&entity_id) {
            None => panic!("Entity: {:?} not registered!", entity_id),
            Some(component_ids) => component_ids.insert(component_id),
        };
        let component_manager = self
            .component_id_to_component_managers
            .entry(component_id)
            .or_insert(Box::new(ComponentManager::new()));
        component_manager.register_entity(entity_id, Rc::new(RefCell::new(component)));
    }

    get_components_from_entity!(get_component A);
    get_components_from_entity!(get_two_components A, B);
    get_components_from_entity!(get_three_components A, B, C);
    get_components_from_entity!(get_four_components A, B, C, D);
    get_components_from_entity!(get_five_components A, B, C, D, E);

    filter_entities_with_components!(filter_component A);
    filter_entities_with_components!(filter_two_components A, B);
    filter_entities_with_components!(filter_three_components A, B, C);
    filter_entities_with_components!(filter_four_components A, B, C, D);
    filter_entities_with_components!(filter_five_components A, B, C, D, E);
}
