use ggez::event;
use ggez::glam::*;
use ggez::graphics;
use ggez::{Context, GameResult};

use moecs::component::Component;
use moecs::entity::{EntityBuilder, EntityManager, Query};
use moecs::system::{System, SystemParam, SystemParamAccessor};
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
    fn execute(entity_manager: &mut EntityManager, _params: &SystemParamAccessor) {
        entity_manager.create_entity(
            EntityBuilder::default()
                .add_component(PositionComponent { x: 0.0, y: 0.0 })
                .add_component(VelocityComponent {
                    x_vel: 0.5,
                    y_vel: 0.5,
                })
                .add_component(DrawComponent {}),
        );
        entity_manager.create_entity(
            EntityBuilder::default()
                .add_component(PositionComponent { x: 200.0, y: 50.0 })
                .add_component(VelocityComponent {
                    x_vel: -0.5,
                    y_vel: 1.0,
                })
                .add_component(DrawComponent {}),
        );
        entity_manager.create_entity(
            EntityBuilder::default()
                .add_component(PositionComponent { x: 175.0, y: 200.0 })
                .add_component(VelocityComponent {
                    x_vel: -0.2,
                    y_vel: -0.2,
                })
                .add_component(DrawComponent {}),
        );
    }
}

#[derive(System)]
struct PhysicsSystem;
impl System for PhysicsSystem {
    fn execute(entity_manager: &mut EntityManager, _params: &SystemParamAccessor) {
        entity_manager
            .filter(Query::with::<PositionComponent>().and::<VelocityComponent>())
            .iter()
            .for_each(|entity_id| {
                let (position, velocity) = entity_manager
                    .get_two_components::<PositionComponent, VelocityComponent>(entity_id);
                let position = position.unwrap();
                let velocity = velocity.unwrap();

                position.borrow_mut().x += velocity.borrow().x_vel;
                position.borrow_mut().y += velocity.borrow().y_vel;
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
    fn execute(entity_manager: &mut EntityManager, params: &SystemParamAccessor) {
        let canvas_param = params.get_param::<CanvasParam>().unwrap();
        let canvas_param = &mut canvas_param.borrow_mut();
        let canvas = &mut canvas_param.canvas;

        entity_manager
            .filter(Query::with::<PositionComponent>().and::<DrawComponent>())
            .iter()
            .for_each(|entity_id| {
                let position = entity_manager.get_component::<PositionComponent>(entity_id);
                let position = position.unwrap();

                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect {
                            x: position.borrow().x,
                            y: position.borrow().y,
                            w: 50.0,
                            h: 50.0,
                        })
                        .color([0.0, 0.0, 0.0, 1.0]),
                );
            });
    }
}

struct GameState {
    engine: moecs::Engine,
    logic_systems: Vec<u64>,
    render_systems: Vec<u64>,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let mut engine = Engine::default();
        engine.execute_now::<CreateEntitiesSystem>(&SystemParamAccessor::default());
        let logic_systems = vec![engine.register_system::<PhysicsSystem>()];
        let render_systems = vec![engine.register_system::<RenderSystem>()];

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
            .execute_systems(&self.logic_systems, &SystemParamAccessor::default());
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(context, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        self.engine.execute_systems(
            &self.render_systems,
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
