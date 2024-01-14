use ggez::event;
use ggez::glam::*;
use ggez::graphics;
use ggez::{Context, GameResult};
use rand::Rng;
use std::sync::{Arc, RwLock};

use moecs::component::{Component, ComponentBundle};
use moecs::entity::{EntityManager, Query};
use moecs::system::{System, SystemGroup, SystemParam, SystemParamAccessor};
use moecs::Engine;

#[derive(Component)]
struct PositionComponent {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct VelocityComponent {
    x_vel: f32,
    y_vel: f32,
}

#[derive(Component)]
struct DrawComponent {}

#[derive(System)]
struct CreateEntitiesSystem;
impl System for CreateEntitiesSystem {
    fn execute(entity_manager: Arc<RwLock<EntityManager>>, _params: Arc<SystemParamAccessor>) {
        for _ in 0..1_000 {
            let mut rng = rand::thread_rng();
            entity_manager.write().unwrap().create_entity(
                ComponentBundle::default()
                    .add_component(PositionComponent {
                        x: rng.gen::<f32>() * 800.0,
                        y: rng.gen::<f32>() * 600.0,
                    })
                    .add_component(VelocityComponent {
                        x_vel: rng.gen::<f32>() * 2.0 - 1.0,
                        y_vel: rng.gen::<f32>() * 2.0 - 1.0,
                    })
                    .add_component(DrawComponent {}),
            );
        }
    }
}

#[derive(System)]
struct PhysicsSystem;
impl System for PhysicsSystem {
    fn execute(entity_manager: Arc<RwLock<EntityManager>>, _params: Arc<SystemParamAccessor>) {
        entity_manager
            .read()
            .unwrap()
            .filter(
                Query::default()
                    .with::<PositionComponent>()
                    .with::<VelocityComponent>(),
            )
            .iter()
            .for_each(|result| {
                let position = result.get_component::<PositionComponent>().unwrap();
                let velocity = result.get_component::<VelocityComponent>().unwrap();

                position.write().unwrap().x += velocity.read().unwrap().x_vel;
                position.write().unwrap().y += velocity.read().unwrap().y_vel;
            });
    }
}

#[derive(SystemParam)]
struct CanvasParam<'a> {
    canvas: &'a mut graphics::Canvas,
}
#[derive(System)]
struct RenderSystem;
impl System for RenderSystem {
    fn execute(entity_manager: Arc<RwLock<EntityManager>>, params: Arc<SystemParamAccessor>) {
        let canvas_param = params.get_param::<CanvasParam>().unwrap();
        let canvas_param = &mut canvas_param.write().unwrap();
        let canvas = &mut canvas_param.canvas;

        entity_manager
            .read()
            .unwrap()
            .filter(
                Query::default()
                    .with::<PositionComponent>()
                    .with::<DrawComponent>(),
            )
            .iter()
            .for_each(|result| {
                let position = result.get_component::<PositionComponent>().unwrap();

                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect {
                            x: position.read().unwrap().x,
                            y: position.read().unwrap().y,
                            w: 10.0,
                            h: 10.0,
                        })
                        .color([0.0, 0.0, 0.0, 1.0]),
                );
            });
    }
}

struct GameState {
    engine: moecs::Engine,
    logic_systems: u32,
    render_systems: u32,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let mut engine = Engine::default();
        let startup_systems = engine.register_system_group(
            SystemGroup::new_sequential_group().register::<CreateEntitiesSystem>(),
        );
        engine.execute_group(startup_systems, SystemParamAccessor::default());

        let logic_systems = engine
            .register_system_group(SystemGroup::new_sequential_group().register::<PhysicsSystem>());
        let render_systems = engine
            .register_system_group(SystemGroup::new_parallel_group().register::<RenderSystem>());

        Ok(GameState {
            engine,
            logic_systems,
            render_systems,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.engine
            .execute_group(self.logic_systems, SystemParamAccessor::default());
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(context, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        self.engine.execute_group(
            self.render_systems,
            SystemParamAccessor::default().add_param(CanvasParam {
                canvas: &mut canvas,
            }),
        );
        canvas.finish(context)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("moecs_ggez_example", "mccloskeybr");
    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new()?;
    event::run(ctx, event_loop, state)
}
