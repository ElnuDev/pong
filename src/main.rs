use hecs::*;
use ggez::*;

use rapier2d::na::Vector2;
use rapier2d::dynamics::{JointSet, RigidBodySet, IntegrationParameters};
use rapier2d::geometry::{BroadPhase, NarrowPhase, ColliderSet};
use rapier2d::pipeline::PhysicsPipeline;

mod components;
use components::*;

mod systems;
use systems::*;

struct MainState {
    world: World,
}

impl MainState {
    fn new () -> MainState {
        MainState {
            world: World::new(),
        }
    }
}

impl event::EventHandler for MainState {
    fn update (&mut self, context: &mut Context) -> GameResult {
        system_physics(&mut self.world, context);

        Ok(())
    }

    fn draw (&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        system_rect_draw(&mut self.world, context);

        graphics::present(context)?;

        Ok(())
    }
}

fn main () -> GameResult {
    // Set up

    let context_builder = ContextBuilder::new("ecs-testing", "Elnu");
    let (mut context, mut event_loop) = context_builder.build()?;

    graphics::set_window_title(&context, "ecs-testing");

    let mut main_state = MainState::new();
    
    // Create entities

    let entity = main_state.world.spawn((
        Transform {
            position: nalgebra::Point2::new(0.0, 0.0),
        },
        Rect {
            rect: graphics::Rect::new(0.0, 0.0, 32.0, 32.0),
        },
        Physics {
            velocity: nalgebra::Vector2::new(5.0, 0.0),
        }
    ));

    // Run event loop
    
    event::run(&mut context, &mut event_loop, &mut main_state)?;

    Ok(())
}