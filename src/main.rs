use hecs::*;
use ggez::*;

use rapier2d::dynamics::RigidBodyBuilder;
use rapier2d::geometry::ColliderBuilder;

mod components;
use components::*;

mod systems;
use systems::*;

mod physics_world;
use physics_world::*;

struct MainState {
    world: World,
    physics_world: PhysicsWorld,
}

impl MainState {
    fn new () -> MainState {
        MainState {
            world: World::new(),
            physics_world: PhysicsWorld::new(),
        }
    }
}

impl event::EventHandler for MainState {
    fn update (&mut self, context: &mut Context) -> GameResult {
        system_physics(&mut self.physics_world, context);

        Ok(())
    }

    fn draw (&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        system_rect_draw(&mut self.world, &mut self.physics_world, context);

        graphics::present(context)?;

        Ok(())
    }
}

fn main () -> GameResult {
    // Set up ggez

    let context_builder = ContextBuilder::new("Pong", "Elnu");
    let (mut context, mut event_loop) = context_builder.build()?;

    graphics::set_window_title(&context, "Pong");

    let mut main_state = MainState::new();
    
    // Set up physics

    // Create entities

    let rigid_body_handle = main_state.physics_world.rigid_body_set.insert(
        RigidBodyBuilder::new_dynamic().build()
    );
    main_state.physics_world.collider_set.insert(
        ColliderBuilder::ball(0.5).build(),
        rigid_body_handle,
        &mut main_state.physics_world.rigid_body_set
    );

    let entity = main_state.world.spawn((
        RectComponent {
            rect: graphics::Rect::new(0.0, 0.0, 32.0, 32.0),
        },

        RigidBodyComponent {
            rigid_body_handle,
        }
    ));

    // Run event loop
    
    event::run(&mut context, &mut event_loop, &mut main_state)?;

    Ok(())
}