use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

use ggez::event;
use ggez::glam::*;
use ggez::graphics;
use ggez::{Context, GameResult};

use pecs::manager::{EntityManager, EntityQuery};
use pecs::{Component, Engine, EntityBuilder, System, SystemParam, SystemParamAccessor};

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

struct PhysicsSystem {}
impl System for PhysicsSystem {
    fn execute(&self, entity_manager: &mut EntityManager, _params: &SystemParamAccessor) {
        entity_manager
            .filter(
                EntityQuery::default()
                    .with_component::<PositionComponent>()
                    .with_component::<VelocityComponent>(),
            )
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
struct CanvasParam {
    canvas: Rc<RefCell<Option<graphics::Canvas>>>,
}
struct RenderSystem {}
impl System for RenderSystem {
    fn execute(&self, entity_manager: &mut EntityManager, params: &SystemParamAccessor) {
        let mut canvas_param = params.get_param::<CanvasParam>();
        let canvas_param = canvas_param.as_mut().unwrap();
        let canvas = &canvas_param.borrow_mut().canvas;
        let mut canvas = canvas.borrow_mut();

        entity_manager
            .filter(
                EntityQuery::default()
                    .with_component::<PositionComponent>()
                    .with_component::<DrawComponent>(),
            )
            .iter()
            .for_each(|entity_id| {
                let position = entity_manager.get_component::<PositionComponent>(entity_id);
                let position = position.unwrap();

                canvas.as_mut().unwrap().draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect {
                            x: position.borrow().x,
                            y: position.borrow().y,
                            w: 100.0,
                            h: 100.0,
                        })
                        .color([0.0, 0.0, 0.0, 1.0]),
                );
            });
    }
}

struct GameState {
    engine: pecs::Engine,
    logic_systems: Vec<TypeId>,
    render_systems: Vec<TypeId>,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let mut engine = Engine::default();
        let logic_systems = vec![engine.register_system(PhysicsSystem {})];
        let render_systems = vec![engine.register_system(RenderSystem {})];

        engine.create_entity(
            EntityBuilder::default()
                .add_component(PositionComponent { x: 0.0, y: 0.0 })
                .add_component(VelocityComponent {
                    x_vel: 0.5,
                    y_vel: 0.5,
                })
                .add_component(DrawComponent {}),
        );

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
        let canvas = Rc::new(RefCell::new(Some(graphics::Canvas::from_frame(
            context,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        ))));

        self.engine.execute_systems(
            &self.render_systems,
            SystemParamAccessor::default().add_param(CanvasParam {
                canvas: canvas.clone(),
            }),
        );

        let canvas = Rc::try_unwrap(canvas).unwrap();
        canvas.take().unwrap().finish(context)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pecs_ggez_example", "mccloskeybr");
    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new()?;
    event::run(ctx, event_loop, state)
}
