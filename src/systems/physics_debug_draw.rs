
use ggez::{
    graphics,
    Context,
};
use hecs::*;
use nalgebra;

use crate::physics_world::PhysicsWorld;
use crate::components::*;

use crate::settings;

use rapier2d::geometry::
{
    ShapeType,
    Segment
};

pub fn system_physics_debug_draw(world: &mut World, physics_world: &mut PhysicsWorld, context: &mut Context) {
    for (_id, (rigid_body,)) in &mut world.query::<(&RigidBodyComponent,)>() {
        let rigid_body = physics_world.rigid_body_set.get(rigid_body.rigid_body_handle).unwrap();
        let draw_param = graphics::DrawParam::default()
            .dest(rapier2d::na::Point2::from(rigid_body.position.translation.vector) * settings::UNIT_SIZE)
            .rotation(rigid_body.position.rotation.angle());
        for collider_handle in rigid_body.colliders() {
            let collider = physics_world.collider_set.get(*collider_handle).unwrap();
            let shape = collider.shape();
            let draw_mode = graphics::DrawMode::Stroke(graphics::StrokeOptions::DEFAULT);
            let color = graphics::Color::new(0.0, 1.0, 0.0, 1.0);
            match shape.shape_type() {
                ShapeType::Ball => {
                    let ball = shape.as_ball().unwrap();
                    let mesh = graphics::Mesh::new_circle(
                        context,
                        draw_mode,
                        nalgebra::Point2::new(0.0, 0.0),
                        ball.radius * settings::UNIT_SIZE,
                        0.1, color
                    ).unwrap();
                    graphics::draw(context, &mesh, draw_param).unwrap();
                },
                ShapeType::Polygon => {
                    unimplemented!();
                },
                ShapeType::Cuboid => {
                    let cuboid = shape.as_cuboid().unwrap();
                    let mesh = graphics::Mesh::new_rectangle(
                        context,
                        draw_mode,
                        graphics::Rect::new(
                            -cuboid.half_extents.x * settings::UNIT_SIZE,
                            -cuboid.half_extents.y * settings::UNIT_SIZE,
                            cuboid.half_extents.x * 2.0 * settings::UNIT_SIZE,
                            cuboid.half_extents.y * 2.0 * settings::UNIT_SIZE
                        ),
                        color
                    ).unwrap();
                    graphics::draw(context, &mesh, draw_param).unwrap();
                },
                ShapeType::Capsule => {
                    let capsule = shape.as_capsule().unwrap();
                    let mesh = graphics::Mesh::new_circle(
                        context,
                        draw_mode,
                        capsule.segment.a * settings::UNIT_SIZE,
                        capsule.radius * settings::UNIT_SIZE,
                        0.1, color
                    ).unwrap();
                    graphics::draw(context, &mesh, draw_param).unwrap();

                    let mesh = graphics::Mesh::new_circle(
                        context,
                        draw_mode,
                        capsule.segment.b * settings::UNIT_SIZE,
                        capsule.radius * settings::UNIT_SIZE,
                        0.1, color
                    ).unwrap();
                    graphics::draw(context, &mesh, draw_param).unwrap();

                    let mesh = graphics::Mesh::new_line(
                       context,
                       &[(capsule.segment.a + nalgebra::Vector2::new(0.0, capsule.radius)) * settings::UNIT_SIZE, (capsule.segment.b + nalgebra::Vector2::new(0.0, capsule.radius)) * settings::UNIT_SIZE],
                       1.0,
                       color 
                    ).unwrap();
                    graphics::draw(context, &mesh, draw_param).unwrap();

                    let mesh = graphics::Mesh::new_line(
                        context,
                        &[(capsule.segment.a - nalgebra::Vector2::new(0.0, capsule.radius)) * settings::UNIT_SIZE, (capsule.segment.b - nalgebra::Vector2::new(0.0, capsule.radius)) * settings::UNIT_SIZE],
                        1.0,
                        color 
                     ).unwrap();
                     graphics::draw(context, &mesh, draw_param).unwrap();
                },
                ShapeType::Segment => {
                    let segment = shape.downcast_ref::<Segment>().unwrap();
                    let mesh = graphics::Mesh::new_line(
                       context,
                       &[segment.a * settings::UNIT_SIZE, segment.b * settings::UNIT_SIZE],
                       1.0,
                       color 
                    ).unwrap();
                    graphics::draw(context, &mesh, draw_param).unwrap();
                },
                ShapeType::Triangle => {
                    unimplemented!();
                },
                ShapeType::Trimesh => {
                    let trimesh = shape.as_trimesh().unwrap();
                    let scaled_vertices = trimesh.vertices()
                        .iter()
                        .map(|vertex| vertex * settings::UNIT_SIZE)
                        .collect::<Vec<nalgebra::Point2<f32>>>();
                    let indices = trimesh.indices();
                    for index in indices {
                        let mesh = graphics::Mesh::new_polygon(
                            context,
                            draw_mode,
                            &vec![
                                scaled_vertices[index.x as usize],
                                scaled_vertices[index.y as usize],
                                scaled_vertices[index.z as usize]
                            ],
                            color
                        ).unwrap();
                        graphics::draw(context, &mesh, draw_param).unwrap();
                    }
                },
                ShapeType::HeightField => {
                    unimplemented!();
                },
            };
        }
    }
}