use std::any::TypeId;

use pecs::component::Component;
use pecs::engine::Engine;
use pecs::system::System;
use pecs::manager::entity_manager::EntityManager;

struct SomeComponent {
    x: u32,
}
impl Component for SomeComponent {}

struct SomeSystem {}
impl System for SomeSystem {
    fn execute(&self, entity_manager: &mut EntityManager) {
        let entities = entity_manager.apply_filter(&[TypeId::of::<SomeComponent>()]);
        entities.iter().for_each(|entity_id| {
            let component = entity_manager.get_component_for_entity::<SomeComponent>(*entity_id);
            println!("value: {}", component.unwrap().x);
        });
    }
}

fn main() {
    let mut engine = Engine::default();
    engine.register_system(SomeSystem {});

    {
        let entity_id = engine.entity_manager().create_entity();
        engine.entity_manager().add_component_to_entity(entity_id, SomeComponent { x: 5 });
    }

    for _ in 0..3 {
        engine.execute_systems();
    }
}
