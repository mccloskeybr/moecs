use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::component::{Component, ComponentManager};
use crate::entity::EntityBuilder;

#[derive(Default)]
pub struct EntityManager {
    next_entity_id: u32,
    entity_id_to_component_ids: HashMap<u32, HashSet<u64>>,
    component_id_to_component_managers: HashMap<u64, Box<ComponentManager<dyn Component>>>,
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
                    .get(&$component::property_id())
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

impl EntityManager {
    pub fn create_entity(&mut self, entity_builder: &EntityBuilder) -> u32 {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entity_id_to_component_ids
            .insert(entity_id, HashSet::new());

        entity_builder
            .get_components()
            .iter()
            .for_each(|component| {
                let component_id: u64 = component.borrow().self_property_id();
                self.entity_id_to_component_ids
                    .get_mut(&entity_id)
                    .map(|component_ids| component_ids.insert(component_id));
                let component_manager = self
                    .component_id_to_component_managers
                    .entry(component_id)
                    .or_insert(Box::new(ComponentManager::new()));
                component_manager.register_entity(entity_id, component.clone());
            });

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

    pub fn filter(&self, query: &EntityQuery) -> Vec<u32> {
        let mut matches: HashSet<u32> = self.entity_id_to_component_ids.keys().copied().collect();
        query.get_components().iter().for_each(|component_property_id| {
            matches = matches
                .intersection(
                    &self
                        .component_id_to_component_managers
                        .get(component_property_id)
                        .map_or(HashSet::new(), |component_manager| {
                            component_manager.get_all_registered_entities()
                        }),
                )
                .copied()
                .collect();
        });
        matches.iter().copied().collect()
    }

    get_components_from_entity!(get_component A);
    get_components_from_entity!(get_two_components A, B);
    get_components_from_entity!(get_three_components A, B, C);
    get_components_from_entity!(get_four_components A, B, C, D);
    get_components_from_entity!(get_five_components A, B, C, D, E);
}

#[derive(Default)]
pub struct EntityQuery {
    components: Vec<u64>,
}

impl EntityQuery {
    pub fn with_component<T: 'static + Component>(&mut self) -> &mut EntityQuery {
        self.components.push(T::property_id());
        self
    }

    pub fn get_components(&self) -> &Vec<u64> {
        &self.components
    }
}
