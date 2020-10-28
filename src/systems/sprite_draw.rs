use ggez::*;
use hecs::*;
use crate::physics_world::PhysicsWorld;
use crate::components::*;

pub fn system_sprite_draw(world: &mut World, physics_world: &mut PhysicsWorld, context: &mut Context) {
    for (id, (rigid_body, sprite)) in &mut world.query::<(&RigidBodyComponent, &SpriteComponent)>() {
        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = rapier2d::na::Point2::from(physics_world.rigid_body_set.get(rigid_body.rigid_body_handle).unwrap().position.translation.vector).into();
        graphics::draw(context, &sprite.image, draw_param).unwrap();
    }
}