use ggez::*;
use hecs::*;
use crate::physics_world::PhysicsWorld;
use crate::components::*;

use crate::settings;

pub fn system_sprite_draw(world: &mut World, physics_world: &mut PhysicsWorld, context: &mut Context) {
    for (id, (rigid_body, sprite)) in &mut world.query::<(&RigidBodyComponent, &SpriteComponent)>() {
        let mut draw_param = graphics::DrawParam::default()
            .scale(nalgebra::Vector2::new(
                settings::UNIT_SIZE / settings::PIXELS_PER_UNIT,
                settings::UNIT_SIZE / settings::PIXELS_PER_UNIT
            ))
            .offset(ggez::nalgebra::Point2::new(0.5, 0.5));
        draw_param.dest = (rapier2d::na::Point2::from(physics_world.rigid_body_set.get(rigid_body.rigid_body_handle).unwrap().position.translation.vector) * settings::UNIT_SIZE).into();
        graphics::draw(context, &sprite.image, draw_param).unwrap();
    }
}