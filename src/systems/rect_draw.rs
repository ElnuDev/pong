use ggez::*;
use hecs::*;
use crate::physics_world::PhysicsWorld;
use crate::components::*;

pub fn system_rect_draw(world: &mut World, physics_world: &mut PhysicsWorld, context: &mut Context) {
    for (id, (rigid_body, rect)) in &mut world.query::<(&RigidBodyComponent, &RectComponent)>() {
        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = rapier2d::na::Point2::from(physics_world.rigid_body_set.get(rigid_body.rigid_body_handle).unwrap().position.translation.vector).into();
        let mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), rect.rect, graphics::WHITE).unwrap();
        graphics::draw(context, &mesh, draw_param).unwrap();
    }
}