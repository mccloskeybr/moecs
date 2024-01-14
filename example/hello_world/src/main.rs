#![allow(dead_code)]
#![allow(unused_variables)]

use moecs::component::{Component, ComponentBundle};
use moecs::entity::{EntityManager, Query};
use moecs::system::{System, SystemGroup, SystemParamAccessor};
use moecs::Engine;

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
struct PhysicsSystem;
impl System for PhysicsSystem {
    fn execute(entity_manager: &mut EntityManager, params: &SystemParamAccessor) {
        entity_manager
            .filter(Query::default().with::<PositionComponent>().with::<VelocityComponent>())
            .iter()
            .for_each(|result| {
                let position = result.get_component::<PositionComponent>().unwrap();
                let velocity = result.get_component::<VelocityComponent>().unwrap();

                position.write().unwrap().x += velocity.read().unwrap().x_vel;
                position.write().unwrap().y += velocity.read().unwrap().y_vel;

                println!("Entity: {} has position: {:?}", result.entity_id(), position);
            });
    }
}

#[derive(System)]
struct CreateEntitiesSystem;
impl System for CreateEntitiesSystem {
    fn execute(entity_manager: &mut EntityManager, params: &SystemParamAccessor) {
        entity_manager.create_entity(
            ComponentBundle::default()
                .add_component(PositionComponent { x: 0, y: 0 })
                .add_component(VelocityComponent { x_vel: 2, y_vel: 1 }),
        );
        entity_manager.create_entity(
            ComponentBundle::default()
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
    let startup_systems = engine.register_system_group(
        SystemGroup::default()
            .register::<CreateEntitiesSystem>()
            .clone(),
    );
    engine.execute_group(startup_systems, &SystemParamAccessor::default());

    let update_systems =
        engine.register_system_group(SystemGroup::default().register::<PhysicsSystem>().clone());
    for i in 0..5 {
        engine.execute_group(update_systems, &SystemParamAccessor::default());
    }
}
