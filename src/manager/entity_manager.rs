use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

use crate::component::Component;

pub struct EntityManager {
    next_entity_id: u32,
    component_to_entity_ids: HashMap<TypeId, HashSet<u32>>,
    entity_id_to_components: HashMap<u32, HashMap<TypeId, Box<dyn Component>>>,
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            next_entity_id: 0,
            component_to_entity_ids: HashMap::new(),
            entity_id_to_components: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> u32 {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entity_id_to_components
            .insert(entity_id, HashMap::new());
        println!("Entity created: {:?}.", entity_id);
        entity_id
    }

    pub fn delete_entity(&mut self, entity_id: u32) {
        for set in self.component_to_entity_ids.values_mut() {
            set.retain(|curr_entity_id| *curr_entity_id != entity_id);
        }
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: u32,
        component: T,
    ) {
        let component_id: TypeId = component.type_id();
        match self.entity_id_to_components.get_mut(&entity_id) {
            None => panic!("Entity: {:?} not registered!", entity_id),
            Some(components) => components.insert(component_id, Box::new(component)),
        };
        let entities = self
            .component_to_entity_ids
            .entry(component_id)
            .or_default();
        entities.insert(entity_id);
    }

    pub fn get_component_for_entity<T: 'static + Component>(
        &mut self,
        entity_id: u32,
    ) -> Option<&mut T> {
        let components = self.entity_id_to_components.get_mut(&entity_id);
        if components.is_none() {
            panic!("Entity: {:?} not registered!", entity_id);
        }
        let component = components.unwrap().get_mut(&TypeId::of::<T>());
        component.map(|component| component.as_mut_any().downcast_mut::<T>().unwrap())
    }

    pub fn apply_filter(&self, filter: &[TypeId]) -> Vec<u32> {
        let mut matches: Option<HashSet<u32>> = None;
        for component in filter {
            let entities: HashSet<u32> = match self.component_to_entity_ids.get(component) {
                None => panic!(
                    "Attempt to reference unregistered component: {:?}!",
                    component
                ),
                Some(entities) => entities.clone(),
            };
            matches = Some(match matches {
                None => entities,
                Some(matches) => matches.intersection(&entities).copied().collect(),
            });
        }
        matches.unwrap_or_default().iter().copied().collect()
    }
}
