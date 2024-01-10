#![allow(dead_code)]
#![allow(unused_variables)]

use pecs::component::Component;
use pecs::entity::{EntityBuilder, EntityManager, Query};
use pecs::system::{System, SystemParamAccessor};
use pecs::Engine;

#[derive(Component, Debug)]
struct PositionComponent {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
struct VelocityComponent {
    x_vel: i32,
    y_vel: i32,
}

#[derive(System)]
struct PhysicsSystem {}
impl System for PhysicsSystem {
    fn execute(&self, entity_manager: &mut EntityManager, params: &SystemParamAccessor) {
        entity_manager
            .filter(
                Query::default()
                    .with::<PositionComponent>()
                    .with::<VelocityComponent>(),
            )
            .iter()
            .for_each(|entity_id| {
                let (position, velocity) = entity_manager
                    .get_two_components::<PositionComponent, VelocityComponent>(entity_id);
                let position = position.unwrap();
                let velocity = velocity.unwrap();

                position.borrow_mut().x += velocity.borrow_mut().x_vel;
                position.borrow_mut().y += velocity.borrow_mut().y_vel;

                println!("Entity: {} has position: {:?}", entity_id, position);
            });
    }
}

#[derive(System)]
struct CreateEntitiesSystem {}
impl System for CreateEntitiesSystem {
    fn execute(&self, entity_manager: &mut EntityManager, params: &SystemParamAccessor) {
        entity_manager.create_entity(
            EntityBuilder::default()
                .add_component(PositionComponent { x: 0, y: 0 })
                .add_component(VelocityComponent { x_vel: 2, y_vel: 1 }),
        );
        entity_manager.create_entity(
            EntityBuilder::default()
                .add_component(PositionComponent { x: 0, y: 0 })
                .add_component(VelocityComponent {
                    x_vel: -1,
                    y_vel: -1,
                }),
        );
    }
}

fn main() {
    let mut engine = Engine::default();
    let startup_systems = vec![engine.register_system(CreateEntitiesSystem {})];
    engine.execute_systems(&startup_systems, &SystemParamAccessor::default());
    engine.deregister_systems(&startup_systems);

    let update_systems = vec![engine.register_system(PhysicsSystem {})];
    for i in 0..5 {
        engine.execute_systems(&update_systems, &SystemParamAccessor::default());
    }
}
