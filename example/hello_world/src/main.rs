#![allow(dead_code)]
#![allow(unused_variables)]

use pecs::component::Component;
use pecs::engine::Engine;
use pecs::manager::entity_manager::EntityManager;
use pecs::system::System;

#[derive(Debug)]
struct PositionComponent {
    x: i32,
    y: i32,
}
impl Component for PositionComponent {}

#[derive(Debug)]
struct VelocityComponent {
    x_vel: i32,
    y_vel: i32,
}
impl Component for VelocityComponent {}

#[derive(Debug)]
struct KillCountdownComponent {
    time_to_live_countdown: u32,
}
impl KillCountdownComponent {
    pub fn tick(&mut self) {
        self.time_to_live_countdown -= 1;
    }

    pub fn is_dead(&self) -> bool {
        self.time_to_live_countdown == 0
    }
}
impl Component for KillCountdownComponent {}

struct PrintPositionSystem {}
impl System for PrintPositionSystem {
    fn execute(&self, entity_manager: &mut EntityManager) {
        let entities = entity_manager.filter_component::<PositionComponent>();
        entities.iter().for_each(|entity_id| {
            let position = entity_manager.get_component::<PositionComponent>(entity_id);
            let position = position.unwrap();

            println!("Entity: {} has position: {:?}", entity_id, position);
        });
    }
}

struct PhysicsSystem {}
impl System for PhysicsSystem {
    fn execute(&self, entity_manager: &mut EntityManager) {
        let entities =
            entity_manager.filter_two_components::<PositionComponent, VelocityComponent>();
        entities.iter().for_each(|entity_id| {
            let (position, velocity) = entity_manager
                .get_two_components::<PositionComponent, VelocityComponent>(entity_id);
            let position = position.unwrap();
            let velocity = velocity.unwrap();

            position.borrow_mut().x += velocity.borrow_mut().x_vel;
            position.borrow_mut().y += velocity.borrow_mut().y_vel;
        });
    }
}

struct KillCountdownSystem {}
impl System for KillCountdownSystem {
    fn execute(&self, entity_manager: &mut EntityManager) {
        let entities = entity_manager.filter_component::<KillCountdownComponent>();
        entities.iter().for_each(|entity_id| {
            let kill_countdown = entity_manager.get_component::<KillCountdownComponent>(entity_id);
            let kill_countdown = kill_countdown.unwrap();

            kill_countdown.borrow_mut().tick();
            if kill_countdown.borrow().is_dead() {
                entity_manager.delete_entity(*entity_id);
            }
        });
    }
}

fn main() {
    let mut engine = Engine::default();
    engine.register_system(PrintPositionSystem {});
    engine.register_system(PhysicsSystem {});
    engine.register_system(KillCountdownSystem {});

    {
        let entity_id = engine.entity_manager().create_entity();
        engine
            .entity_manager()
            .add_component_to_entity(entity_id, PositionComponent { x: 0, y: 0 });
        engine
            .entity_manager()
            .add_component_to_entity(entity_id, VelocityComponent { x_vel: 2, y_vel: 1 });
        engine.entity_manager().add_component_to_entity(
            entity_id,
            KillCountdownComponent {
                time_to_live_countdown: 3,
            },
        );
    }
    {
        let entity_id = engine.entity_manager().create_entity();
        engine
            .entity_manager()
            .add_component_to_entity(entity_id, PositionComponent { x: 0, y: 0 });
        engine.entity_manager().add_component_to_entity(
            entity_id,
            VelocityComponent {
                x_vel: -1,
                y_vel: -1,
            },
        );
    }

    for _ in 0..5 {
        engine.execute_systems();
    }
}
