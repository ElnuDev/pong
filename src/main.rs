use hecs::*;
use ggez::{
    graphics,
    Context,
    ContextBuilder,
    event,
    GameResult,
    conf
};

use rapier2d::dynamics::RigidBodyBuilder;
use rapier2d::geometry::ColliderBuilder;

use std::env;
use std::path;

mod components;
use components::*;

mod systems;
use systems::*;

mod physics_world;
use physics_world::*;

mod settings;

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

        system_sprite_draw(&mut self.world, &mut self.physics_world, context);
        system_physics_debug_draw(&mut self.world, &mut self.physics_world, context);

        graphics::present(context)?;

        Ok(())
    }
}

fn main () -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Set up ggez

    let (mut context, mut event_loop) = ContextBuilder::new("Pong", "Elnu")
        .window_setup(conf::WindowSetup::default().title("Pong"))
        .window_mode(conf::WindowMode::default().dimensions(settings::WINDOW_WIDTH, settings::WINDOW_HEIGHT))
        .add_resource_path(resource_dir)
        .build()?;

    let mut main_state = MainState::new();
    
    // Create entities

    // Ball
    {
        let rigid_body_handle = main_state.physics_world.rigid_body_set.insert(
            RigidBodyBuilder::new_dynamic().translation(graphics::drawable_size(&context).0 / settings::UNIT_SIZE / 2.0, 1.0).build()
        );
        main_state.physics_world.collider_set.insert(
            ColliderBuilder::ball(0.5).build(),
            rigid_body_handle,
            &mut main_state.physics_world.rigid_body_set
        );

        main_state.world.spawn((
            //RectComponent {
            //    rect: graphics::Rect::new(0.0, 0.0, 32.0, 32.0),
            //},
            RigidBodyComponent {
                rigid_body_handle,
            },
            SpriteComponent::new(&mut context, "/rust.png"),
        ));
    }

    // Capsule
    {
        let rigid_body_handle = main_state.physics_world.rigid_body_set.insert(
            RigidBodyBuilder::new_dynamic().translation(graphics::drawable_size(&context).0 / settings::UNIT_SIZE / 2.0 + 1.0, -3.0).build()
        );
        main_state.physics_world.collider_set.insert(
            ColliderBuilder::capsule_x(1.0, 0.5).build(),
            rigid_body_handle,
            &mut main_state.physics_world.rigid_body_set
        );

        main_state.world.spawn((
            RigidBodyComponent {
                rigid_body_handle,
            },
            SpriteComponent::new(&mut context, "/rust.png"),
        ));
    }

    // Ground
    {
        let rigid_body_handle = main_state.physics_world.rigid_body_set.insert(
            RigidBodyBuilder::new_static()
            .translation(
                graphics::drawable_size(&context).0 / settings::UNIT_SIZE / 2.0,
                graphics::drawable_size(&context).1 / settings::UNIT_SIZE - 0.625
            )
            .build()
        );
        main_state.physics_world.collider_set.insert(
            ColliderBuilder::cuboid(graphics::drawable_size(&context).0 / settings::UNIT_SIZE / 2.0 - 0.125, 0.5).build(),
            rigid_body_handle,
            &mut main_state.physics_world.rigid_body_set
        );

        main_state.world.spawn((
            RigidBodyComponent {
                rigid_body_handle,
            },
            SpriteComponent::new(&mut context, "/rust.png"),
        ));
    }

    // Run event loop
    
    event::run(&mut context, &mut event_loop, &mut main_state)?;

    Ok(())
}